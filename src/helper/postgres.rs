use sqlx::{postgres::{PgPool, PgPoolOptions}};
use std::env;
use lazy_static::lazy_static;
use async_once::AsyncOnce;

lazy_static! {
    // static ref CONN: Arc<Mutex<PgPool>> = Arc::new(Mutex::new(connect_postgres().await));ef
    pub static ref POOL: AsyncOnce<PgPool> = AsyncOnce::new(async  {
        let client = open_postgres().await;

        client
    });
}
pub async fn open_postgres() -> PgPool {
    let url = env::var("DATABASE_URL")
        .expect("Can't get postgres db url");
    println!("url postgres: {}", url);
    
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(url.as_str())
    .await
    .expect("Can't connect to database");

    pool
}


pub async fn connect_postgres() -> PgPool {
    let _db = POOL.get().await.to_owned();

    _db
}