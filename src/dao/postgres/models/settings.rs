use std::path::PathBuf;
use chrono::offset::Utc;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dao::postgres::schema::ghost::settings;
use dao::common::{SettingsType, DaoBackend};
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
}

impl Default for NewSetting {
    fn default() -> Self {
        NewSetting {
            key: "".to_string(),
            type_: SettingsType::default(),
            value: None,
            created_at: Utc::now().naive_utc(),
            created_by: "system".to_string(), // get from session
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

impl Setting {
    fn initial_db_data() {
        use dao::postgres::schema::ghost::settings::dsl::*;

        // load db settings
        let connection: PgConnection =
            util::establish_connection::<PgConnection>(DaoBackend::Postgres);
        let settings_collection: Vec<Setting> = settings
            .filter(id.gt(0))
            .load::<Setting>(&connection)
            .expect("Error loading settings from db");

        // load and parse default settings
        let mut root_dir: PathBuf = app_util::get_root_dir();
        root_dir.push("src/dao/data/default_settings.json");
        let mut file_content = app_util::get_file_content::<PathBuf>(&root_dir).expect(
            format!(
                "Error get file content, file is {}",
                root_dir.display()
            ).as_str(),
        );
        let default_values_json: Value =
            serde_json::from_str(&file_content).expect("Error de json from file");
        let default_values: Vec<Vec<NewSetting>> = default_values_json
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

                        new_setting
                    })
                    .collect()
            })
            .collect();
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
