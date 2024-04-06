use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
   pub pool: DBPool,
}
use std::sync::{Arc, Mutex};
impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Arc::new(
            Mutex::new(pool.clone()));
        Database { pool }
    }
}
