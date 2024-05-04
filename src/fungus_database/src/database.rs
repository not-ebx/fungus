use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use once_cell::sync::Lazy;
use r2d2::Pool;
use std::env;

type PgPool = Pool<ConnectionManager<PgConnection>>;

static CONNECTION_POOL: Lazy<PgPool> = Lazy::new(|| {
    dotenvy::dotenv().expect(".env file missing");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing in .env file");

    let manager = ConnectionManager::<PgConnection>::new(db_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
});

pub fn get_db() -> r2d2::PooledConnection<ConnectionManager<PgConnection>> {
    CONNECTION_POOL.get().expect("Failed to get a connection from the pool")
}