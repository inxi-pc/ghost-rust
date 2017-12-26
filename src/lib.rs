#![recursion_limit="256"]
#[macro_use] extern crate diesel_infer_schema;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate chrono;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub mod dao;
pub mod util;
pub mod error;