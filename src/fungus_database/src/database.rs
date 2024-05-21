use std::env;
use std::sync::Arc;
use sqlx::postgres::{PgPool, PgPoolOptions};
use once_cell::sync::Lazy;

static DATABASE_POOL: Lazy<Arc<PgPool>> = Lazy::new(|| {
    let database_url= env::var("DATABASE_URL").expect("DATABASE_URL missing in .env file");
    Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(database_url.as_str())
            .expect("cant connec to db lol")
    )
});

//pub async fn get_db() -> PgPool {
pub fn get_db() -> Arc<PgPool> {
    DATABASE_POOL.clone()
}
