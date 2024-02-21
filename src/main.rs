// use crate::path::path;
// use dotenv::dotenv;
// use sqlx::PgPool;
// use tide::http::headers::HeaderValue;
// use tide::security::{CorsMiddleware,Origin};
// use chrono::prelude::*;


// mod path;

// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     dotenv().ok();


//     let now  = Local::now();
//     println!("Waktu Sekarang : {} ", now.to_string());

//     let pool = PgPool::connect(&std::env::var("DATABASE_URL")
//             .expect("DB Config Err"))
//             .await.expect("DB Connection Err");
//     let cors = CorsMiddleware::new()
//             .allow_methods(
//                 "GET, PUT, DELETE, POST, OPTIONS"
//                 .parse::<HeaderValue>()
//                 .unwrap(),
//             )
//             .allow_origin(Origin::from("*"))
//             .allow_credentials(false);
//     tide::log::start();
//     let mut app = tide::with_state(pool.clone());   
//     app.with(cors);
//     path(&mut app);
//     app.listen("192.168.1.17:9001").await?;
//     Ok(())

// }
use std::sync::Mutex;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{HttpResponse, error, App, HttpServer, web};
use dotenv::dotenv;
use fotografi_be::{query, path, config, middleware, PoolState};
use sqlx::pool;
use sqlx::postgres::PgPoolOptions;
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware,Origin};
use chrono::prelude::*;

use sqlx::{PgPool, Pool, Postgres};
// use actix_web::middleware::{Logger, self};

// mod path;
// #[derive(Debug)]
// pub struct PoolState {
//     pub pool: Mutex<Pool<Postgres>>, 
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:12345@127.0.0.1:5432/GienPict").await
        .expect("connection failed!");
    
    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "PUT", "DELETE", "POST", "OPTIONS"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        // custom `Json` extractor configuration
        let json_cfg = web::JsonConfig::default()
            // limit request payload size
            .limit(104857600)
            // only accept text/plain content type
            // .content_type(|mime| mime == mime::TEXT_PLAIN)
            // use custom error handler
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            });

        let pool_state = web::Data::new(PoolState {
            pool: Mutex::new(pool.clone()),
        });

        App::new()
        .app_data(pool_state)
        .wrap(cors)
        .wrap(middleware::SayHi)
        .wrap(Logger::default())
        .configure(config)
    })
    .bind(("10.212.79.40", 9000))?
    .run()
    .await?;

    Ok(())
}

// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     dotenv().ok();


//     let now  = Local::now();
//     println!("Waktu Sekarang : {} ", now.to_string());

//     let pool = PgPool::connect(&std::env::var("DATABASE_URL")
//             .expect("DB Config Err"))
//             .await.expect("DB Connection Err");
//     let cors = CorsMiddleware::new()
//             .allow_methods(
//                 "GET, PUT, DELETE, POST, OPTIONS"
//                 .parse::<HeaderValue>()
//                 .unwrap(),
//             )
//             .allow_origin(Origin::from("*"))
//             .allow_credentials(false);
//     tide::log::start();
//     let mut app = tide::with_state(pool.clone());   
//     app.with(cors);
//     path(&mut app);
//     app.listen("192.168.1.31:9001").await?;
//     Ok(())

// }
