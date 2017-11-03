use dotenv::dotenv;
use std::env;
use r2d2::{self, Pool};
use r2d2_diesel::ConnectionManager;
use diesel::prelude::*;
use super::common::DaoBackend;

pub fn establish_connection<T: Connection>(backend: DaoBackend) -> T {
    let database_url = get_database_url(backend);
    Connection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_pool<T: Connection + 'static>(
    backend: DaoBackend,
) -> Pool<ConnectionManager<T>> {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<T>::new(get_database_url(backend));

    r2d2::Pool::new(config, manager).expect("Failed to create pool")
}

fn get_database_url(backend: DaoBackend) -> String {
    dotenv().ok();
    match backend {
        DaoBackend::Mysql => {
            env::var("MYSQL_DATABASE_URL").expect("MYSQL_DATABASE_URL must be set")
        }
        DaoBackend::Postgres => env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set"),
    }
}

#[test]
fn establish_connection_pool_test() {
    use diesel::pg::PgConnection;
    use diesel::mysql::MysqlConnection;

    establish_connection_pool::<PgConnection>(DaoBackend::Postgres);
    establish_connection_pool::<MysqlConnection>(DaoBackend::Mysql);
}
