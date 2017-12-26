use std::path::PathBuf;
use std::collections::HashMap;
use chrono::offset::Utc;
use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dao::postgres::schema::ghost::settings;
use dao::common;
use dao::common::{DaoBackend, SettingsType, TableInitial};
use dao::util;
use util as app_util;
use serde_json::{self, Value};

#[derive(Queryable, Debug, Default)]
pub struct SettingQueryDTO {
    pub id: Option<i32>,
    pub key: Option<String>,
    pub type_: Option<SettingsType>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "settings"]
pub struct NewSetting {
    pub key: String,
    pub type_: SettingsType,
    pub value: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: NaiveDateTime,
    pub updated_by: String,
}

impl Default for NewSetting {
    fn default() -> Self {
        NewSetting {
            key: "".to_string(),
            type_: SettingsType::default(),
            value: None,
            created_at: Utc::now().naive_utc(),
            created_by: "system".to_string(), // get from session
            updated_at: Utc::now().naive_utc(),
            updated_by: "system".to_string(),
        }
    }
}

#[derive(AsChangeset, Debug, Default)]
#[table_name = "settings"]
pub struct SettingChangeset {
    pub key: Option<String>,
    pub type_: Option<SettingsType>,
    pub value: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "settings"]
pub struct Setting {
    pub id: i32,
    pub key: String,
    pub value: Option<String>,
    pub type_: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

impl TableInitial for Setting {
    fn initial_db_data() {
        use dao::postgres::schema::ghost::settings::dsl::*;

        // load and parse default settings
        let default_json_file_path: PathBuf =
            app_util::get_file_path::<&str>(&common::DEFAULT_SETTINGS_FILE_DIR);
        let file_content = app_util::get_file_content::<PathBuf>(&default_json_file_path)
            .expect("Error get default settings json file content");
        let default_values_json: Value =
            serde_json::from_str(&file_content).expect("Error deserialize json from file");
        let default_values_nest: Vec<Vec<NewSetting>> = default_values_json
            .as_object()
            .unwrap()
            .iter()
            .map(|v| {
                v.1
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|inner_v| {
                        let mut new_setting: NewSetting = NewSetting::default();
                        new_setting.key = inner_v.0.clone();
                        new_setting.type_ = SettingsType::new(v.0);
                        new_setting.value = inner_v
                            .1
                            .get("defaultValue")
                            .cloned()
                            .map(|v| v.as_str().unwrap_or("").to_string());

                        new_setting
                    })
                    .collect()
            })
            .collect();
        let default_values: Vec<NewSetting> =
            default_values_nest.into_iter().flat_map(|v| v).collect();
        let default_values_map: HashMap<String, NewSetting> = default_values
            .into_iter()
            .map(|v| (v.key.clone(), v))
            .collect();

        // load db settings
        let connection: PgConnection =
            util::establish_connection::<PgConnection>(DaoBackend::Postgres);
        let exist_settings: Vec<Setting> = settings
            .filter(id.gt(0))
            .load::<Setting>(&connection)
            .expect("Error loading settings data from db");

        if exist_settings.len() <= 0 {
            // insert all default settings into db
            default_values_map.iter().for_each(|(_, v)| {
                diesel::insert_into(settings)
                    .values(v)
                    .execute(&connection)
                    .expect("Error insert default settings data");
            })
        } else {
            // insert not exist data into db
            exist_settings
                .iter()
                .for_each(|v| match default_values_map.get(&v.key) {
                    Some(insert) => {
                        diesel::insert_into(settings)
                            .values(insert)
                            .execute(&connection)
                            .expect("Error insert default settings data");
                    }
                    _ => {}
                })
        }
    }
}

#[test]
fn factory_test() {
    NewSetting::default();
}

#[test]
fn setting_init() {
    Setting::initial_db_data();
}
