use dao::postgres::schema::ghost::settings;
use chrono::offset::Utc;
use chrono::NaiveDateTime;

// enum is diffcult to use
// #[derive(Debug)]
// pub enum SettingsType {
//     Core,
//     Blog,
//     Theme,
//     App,
//     Private
// }

// impl Default for SettingsType {
//     fn default() -> Self {
//         SettingsType::Core
//     }
// }

#[derive(Queryable)]
pub struct SettingsQueryDTO {
    pub id: Option<i32>,
    pub key: Option<String>,
    pub type_: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>
}

#[derive(Insertable)]
#[table_name="settings"]
pub struct NewSettings {
    pub key: String,
    pub type_: String,
    pub value: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: String
}

impl Default for NewSettings {
    fn default() -> Self {
        NewSettings {
            key: "".to_string(),
            type_: "core".to_string(),
            value: None,
            created_at: Utc::now().naive_utc(),
            created_by: "system".to_string() // get from session
        }
    }
}

#[derive(Identifiable)]
#[table_name="settings"]
pub struct Settings {
    pub id: i32,
    pub key: String,
    pub value: Option<String>,
    pub type_: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>
}

#[test]
fn factory_test() {
    let new_settings = NewSettings::default();
}







