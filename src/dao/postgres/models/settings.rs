use dao::postgres::schema::ghost::settings;
use chrono::offset::Utc;
use chrono::NaiveDateTime;

use diesel::pg::Pg;
use diesel::types::{FromSqlRow, Text};
use diesel::row::Row;
use diesel::expression::AsExpression;
use diesel::expression::helper_types::AsExprOf;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SettingsType {
    Core,
    Blog,
    Theme,
    App,
    Private
}

impl Default for SettingsType {
    fn default() -> Self {
        SettingsType::Core
    }
}

impl fmt::Display for SettingsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SettingsType::Core => write!(f, "core"),
            SettingsType::Blog => write!(f, "blog"),
            SettingsType::Theme => write!(f, "theme"),
            SettingsType::App => write!(f, "app"),
            SettingsType::Private => write!(f, "private")
        }
    }
}

impl FromSqlRow<Text, Pg> for SettingsType {
    fn build_from_row<R: Row<Pg>>(row: &mut R) -> Result<Self, Box<Error + Send + Sync>> {
        match String::build_from_row(row)?.as_ref() {
            "core" => Ok(SettingsType::Core),
            "blog" => Ok(SettingsType::Blog),
            "theme" => Ok(SettingsType::Theme),
            "app" => Ok(SettingsType::App),
            "private" => Ok(SettingsType::Private),
            v => Err(format!("Unknown value {} for SettingsType found", v).into()),
        }
    }
}

impl<'a> AsExpression<Text> for &'a SettingsType {
    type Expression = AsExprOf<String, Text>;

    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}

#[derive(Queryable, Debug)]
pub struct SettingsQueryDTO {
    pub id: Option<i32>,
    pub key: Option<String>,
    pub type_: Option<SettingsType>,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>
}

#[derive(Insertable, Debug)]
#[table_name="settings"]
pub struct NewSettings {
    pub key: String,
    pub type_: Option<SettingsType>,
    pub value: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: String
}

impl Default for NewSettings {
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

#[derive(Identifiable, Debug)]
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
    println!("{:?}", new_settings);
}







