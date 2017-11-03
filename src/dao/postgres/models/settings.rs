use std::path::PathBuf;
use chrono::offset::Utc;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dao::postgres::schema::ghost::settings;
use dao::common::{SettingsType, DaoBackend};
use dao::util;
use util as core_util;
use serde_json::{self, Value};
use serde_json::map::Map;

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

#[derive(Deserialize, Debug)]
struct DefaultSettings {
    pub setting_type: String,
    pub settings_collection: Vec<DefaultSettingsElement>,
} 

#[derive(Deserialize, Debug)]
struct DefaultSettingsElement {
    pub key: String,
    pub value_collection: Vec<DefaultSettingsElementValue>,
}

#[derive(Deserialize, Debug)]
struct DefaultSettingsElementValue {
    pub key: String,
    pub value: String,
}

impl Setting {
    pub fn init() -> Map {
        use dao::postgres::schema::ghost::settings::dsl::*;
        // load db settings
        let connection: PgConnection = util::establish_connection::<PgConnection>(DaoBackend::Postgres);
        let settings_collection: Vec<Setting> = settings
            .filter(id.gt(0))
            .load::<Setting>(&connection)
            .expect("Error loading settings");

        // load default settings
        let mut root_dir: PathBuf = core_util::get_root_dir();
        let mut file_content = String::new();
        let mut default_settings = Map::new();
        root_dir.push("src/dao/data/default_settings.json");
        match core_util::load_file_content::<PathBuf>(&root_dir) {
            Ok(content) => file_content = content,
            Err(ref err) => println!("{}", err),
        }
        match serde_json::from_str::<Map<String, DefaultSettings>>(&file_content) {
            Ok(content) => default_settings = content,
            Err(ref err) => println!("{}", err),
        }

        default_settings
    }
}

#[test]
fn factory_test() {
    NewSetting::default();
}

#[test]
fn setting_init() {
    let value: Value = Setting::init();
    println!("{:?}", value.get(SettingsType::Core.to_string()));
}
