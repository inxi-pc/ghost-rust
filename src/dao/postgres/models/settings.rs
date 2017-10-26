use super::schema::settings;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Settings {
    pub id: i32,
    pub key: String,
    pub type: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: NaiveDateTime,
    pub updated_by: String
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
    pub value: String,
    pub type: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: NaiveDateTime,
    pub updated_byL String
}






