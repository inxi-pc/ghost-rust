use super::schema::settings;
use diesel::prelude::*; 
use chrono::offset::Utc;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct SettingsQueryDTO<'a> {
    pub id: Option<i32>,
    pub key: Option<String>,
    pub type_: Option<SettingsType<'a>>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>
}

#[derive(Insertable)]
#[table_name="settings"]
pub struct NewSettings<'a> {
    pub key: String,
    pub type_: Option<SettingsType<'a>>,
    pub value: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: String
}

#[derive(Debug)]
pub enum SettingsType<'a> {
    Core(&'a str),
    Blog(&'a str),
    Theme(&'a str),
    App(&'a str),
    Private(&'a str)
}

impl<'a> Default for SettingsType<'a> {
    fn default() -> Self {
        SettingsType::Core("core")
    }
}

impl<'a> Default for NewSettings<'a> {
    fn default() -> Self {
        NewSettings {
            key: "".to_string(),
            type_: Some(SettingsType::default()),
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







