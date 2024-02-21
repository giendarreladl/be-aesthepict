use actix_web::web;
use sqlx::{Pool, Postgres, query_as};
use tide::Server;
use crate::handler;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    // .route("/test", web::get().to(tes))
    // .route("/login",web::post().to(handler::login_action))
    // .route("/register",web::post().to(handler::register_action))
    // .route("/logout",web::put().to(handler::logout))
    // .route("/mutasi", web::get().to(handler:: mutasi_actions))
    .route("/reg",web::post().to(handler::register_action))
    .route("/login",web::post().to(handler::login_action))
    .route("/foto",web::get().to(handler::foto_actions))
    .route("/pesan",web::post().to(handler::order_action))
    .route("/orderan",web::get().to(handler::getOrder_actions))
    .route("/daftar_harga",web::get().to(handler::DaftarHarga))
    .route("/amchart",web::get().to(handler::read_chart))
    .route("/update",web::put().to(handler::put_order))
    .route("/deletee",web::delete().to(handler::delete_order))
    ;
}