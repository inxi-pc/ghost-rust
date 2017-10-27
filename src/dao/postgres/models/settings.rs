use super::schema::settings;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct SettingsQueryDTO {
    pub id: Option<i32>,
    pub key: Option<String>,
    pub type: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>
}

#[derive(Insertable)]
#[table_name="settings"]
pub struct NewSettings {
    pub key: String,
    pub type: String,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub created_by: String
}

#[derive(Indentifiable)]
#[table_name="settings"]
pub struct Settings {
    pub id: i32,
    pub key: String,
    pub value: Option<String>,
    pub type: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>
}

impl Settings {
    pub fn findSettings(&self) {

    }
}





