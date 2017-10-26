pub mod models;

pub mod schema {
    infer_schema!("dotenv:PG_DATABASE_URL");
}
