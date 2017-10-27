use super::schema::settings;
use chrono::offset::Utc;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct SettingsQueryDTO {
    pub id: Option<i32>,
    pub key: Option<&str>,
    pub type_: Option<&str>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<&str>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<&str>
}

#[derive(Insertable)]
#[table_name="settings"]
pub struct NewSettings {
    pub key: &str,
    pub type_: &str,
    pub value: Option<&str>,
    pub created_at: NaiveDateTime,
    pub created_by: &str
}

#[derive(Debug)]
pub enum SettingsType {
    Core(&str),
    Blog(&str),
    Theme(&str),
    App(&str),
    Private(&str)
}

impl Default for SettingsType {
    fn default() -> Self {
        SettingsType::Core("core")
    }
}

impl Default for NewSettings {
    fn default() -> Self {
        NewSettings {
            key: "",
            type_: Settings::default(),
            value: "",
            created_at: Utc::now().naive_utc(),
            updated_by: "system"
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







