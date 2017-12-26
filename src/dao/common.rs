use diesel::backend::Backend;
use diesel::types::{FromSqlRow, Text, HasSqlType};
use diesel::row::Row;
use diesel::expression::AsExpression;
use diesel::expression::helper_types::AsExprOf;
use std::error::Error;
use std::fmt;

pub static DEFAULT_SETTINGS_FILE_DIR: &'static str = "src/dao/data/default_settings.json";

// dao Backend type, not be same as diesel defined Backend
#[derive(Debug)]
pub enum DaoBackend {
    Mysql,
    Postgres,
}

// table: settings, field: type
#[derive(Debug)]
pub enum SettingsType {
    Core,
    Blog,
    Theme,
    App,
    Private,
}

impl Default for SettingsType {
    fn default() -> Self {
        SettingsType::Core
    }
}

impl SettingsType {
    pub fn new(st: &str) -> Self {
        match st {
            "core" => SettingsType::Core,
            "blog" => SettingsType::Blog,
            "theme" => SettingsType::Theme,
            "app" => SettingsType::App,
            "private" => SettingsType::Private,
            _ => SettingsType::Core,
        }
    }
}

impl fmt::Display for SettingsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SettingsType::Core => write!(f, "core"),
            SettingsType::Blog => write!(f, "blog"),
            SettingsType::Theme => write!(f, "theme"),
            SettingsType::App => write!(f, "app"),
            SettingsType::Private => write!(f, "private"),
        }
    }
}

impl<DB> FromSqlRow<Text, DB> for SettingsType
where
    DB: Backend<RawValue = [u8]> + HasSqlType<Text>,
{
    fn build_from_row<R: Row<DB>>(row: &mut R) -> Result<Self, Box<Error + Send + Sync>> {
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
        AsExpression::<Text>::as_expression(self.to_string())
    }
}

pub trait TableInitial {

    fn initial_db_data();
}
