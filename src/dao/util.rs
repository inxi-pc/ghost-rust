use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use r2d2::{self, Pool};
use r2d2_diesel::ConnectionManager;
use super::Backend;

pub fn establish_connection<T: Connection>(backend: Backend) -> T {
    let database_url = get_database_url(backend);
    Connection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_connection_pool<T: Connection + 'static>(backend: Backend) -> Pool<ConnectionManager<T>> {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<T>::new(get_database_url(backend));

    r2d2::Pool::new(config, manager).expect("Failed to create pool")
}

fn get_database_url(backend: Backend) -> String {
    dotenv().ok();
    match backend {
        Backend::Mysql => env::var("MYSQL_DATABASE_URL").expect("MYSQL_DATABASE_URL must be set"),
        Backend::Postgres => env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set"),
    }
}

#[test]
fn create_connection_pool_test() {
    use diesel::pg::PgConnection;
    use diesel::mysql::MysqlConnection;
    
    let pool_pg = create_connection_pool::<PgConnection>(Backend::Postgres);
    let pool_mysql = create_connection_pool::<MysqlConnection>(Backend::Mysql);
}
