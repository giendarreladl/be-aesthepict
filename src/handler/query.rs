use futures_util::__private::async_await;
use serde::Deserialize;
// use sqlx::PgPool;
use crate::ws_response;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tide::{http, Body, Request, Response};
////////////////////////////////
use crate::connect_postgres;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{cookie, web, HttpRequest, HttpResponse, Responder};
use cookie::Cookie;
use dotenv::dotenv;
use serde::Serialize;
use serde_json::{Map, Value, json};
use sqlx::{pool, PgPool, Pool, Postgres, Executor};
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use std::collections::HashMap;
use std::str::FromStr;
// use sqlx::{postgres::{PgPool}};
use std::sync::Mutex;
use bigdecimal::BigDecimal;


#[derive(serde::Serialize, Debug, Deserialize)]
pub struct Epass {
    pub password: String,
    pub email: String,
}

#[derive(serde::Serialize, Debug, Deserialize)]
pub struct LoginResult {
    pub status: String,
    pub info: String,
}

#[derive(serde::Serialize, Debug, Deserialize)]
pub struct Log {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(serde::Serialize, Debug, Deserialize)]
pub struct Get {
    pub id: i32,
    pub email: Option<String>,
    pub username: Option<String>,
}

// #[derive(serde::Deserialize, Debug)]
// pub struct LoginForm {
//     pub username: String,
//     pub password: String,
// }

#[derive(serde::Deserialize, Debug)]
pub struct RegisForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct PoolState {
    pub pool: Mutex<Pool<Postgres>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Jenisakun {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Users {
    pub id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Mutasi {
    pub id: i32,
    pub kode_barang: Option<i32>,
    pub warna_barang: Option<String>,
    pub vendor: Option<String>,
    pub local_import: Option<String>,
    pub nama_barang: Option<String>,
    pub satuan: Option<String>,
    pub waktu : Option<String>,
    pub jumlah_barang: Option<i32>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Barang {
    pub id : i32,
    pub kode_barang: Option<String>,
    pub kode_barang_as: Option<String> ,
    pub warna_barang: Option<String> ,
    pub nilai_barang: Option<String> ,
    pub nomor_bukti_penerima: Option<i64> ,
    pub tanggal_penerima: Option<String> ,
    pub no_order: Option<i64> ,
    pub nama_barang: Option<String>, 
    pub jumlah: Option<i64>, 
    pub satuan: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Dokpabean {
    pub id : i32,
    pub kode_barang: Option<String>,
    pub jenis_dokumen: Option<String> ,
    pub nomor_dokumen: Option<i64> ,
    pub tgl_dok_dibuat: Option<String> ,
    pub tgl_dok_masuk: Option<String> ,
    pub nomor_awb: Option<String> ,
    pub nomor_shipment: Option<i64> ,
    pub pemasok: Option<String>, 
    pub lokasi_gudang: Option<String>, 
    pub jenis_valuta: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Pabean {
    pub id : i32,
    pub kode_barang: Option<String>,
    pub jenis_dokumen: Option<String> ,
    pub nomor_dokumen: Option<i64> ,
    pub tgl_dok_dibuat: Option<String> ,
    pub tgl_dok_masuk: Option<String> ,
    pub nomor_awb: Option<String> ,
    pub nomor_shipment: Option<i64> ,
    pub pemasok: Option<String>, 
    pub lokasi_gudang: Option<String>, 
    pub jenis_valuta: Option<String>,
    pub kode_barang_as: Option<String> ,
    pub warna_barang: Option<String> ,
    pub nilai_barang: Option<String> ,
    pub nomor_bukti_penerima: Option<i64> ,
    pub tanggal_penerima: Option<String> ,
    pub no_order: Option<i64> ,
    pub nama_barang: Option<String>, 
    pub jumlah: Option<i64>, 
    pub satuan: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RegistrasiForm {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
} 

#[derive(serde::Deserialize, Debug)]
pub struct RegForm {
    pub email: Option<String>,
} 

#[derive(serde::Deserialize, Debug)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
} 

#[derive(Serialize, Debug ,Deserialize)]
pub struct RegisId {
    pub id:i32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tanggal {
    pub start: String,
    pub end: String,
    pub jenis : String,
} 

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Stks {
    pub bulan: String,
    pub tahun: String,
} 

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Authtoken {
    pub id: String,
    pub token: String,
} 

#[derive(Serialize, Debug ,Deserialize)]
pub struct BlogId {
    pub id:i32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Coun {
    pub tahun: String,
} 

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Barangid {
    pub tipe: String,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Mutasiid {
    pub id: i32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Count {
    pub jumlah: Option<i64>,
    pub tgl_dok_dibuat: Option<String>,
    pub tahun : Option<String>,

}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Totl {
    pub jumlah: Option<i64>,
    pub tgl_dok_dibuat: Option<String>,
    pub tahun : Option<String>,

}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TokenCek {
    pub token: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Statis {
    pub jumlah: Option<i64>,
    pub pemasok: Option<String>,
    pub tgl_dok_dibuat: Option<String>,

}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct cekDatas {
    pub pemasok: Option<i64>,// bytea sementara null
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Jenis {
    pub jenis_dokumen: Option<String>,

}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Vendor {
    pub pemasok : Option<String>,
    pub jumlah : Option<i64>,
}

#[derive(serde::Deserialize,Debug, serde::Serialize)]

pub struct Foto{
pub id: i32,
pub image : Option<String>,
pub judul : Option<String>
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Orders {
pub nama: Option<String>,
pub pilihan_order: Option<String>,
pub no_hp: Option<String>,
pub jadwal_order: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct OrdersPut {
pub id: i32,
pub nama: Option<String>,
pub pilihan_order: Option<String>,
pub no_hp: Option<String>,
pub jadwal_order: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GetOrders {
pub id : i32,
pub nama: Option<String>,
pub pilihan_order: Option<String>,
pub jadwal_order: Option<String>,
pub no_hp: Option<String>,
pub status: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Harga {
pub id : i32,
pub pilihan: Option<String>,
pub pilihan_harga: Option<String>,
}

#[derive(serde::Serialize, Debug ,serde::Deserialize)]
pub struct Chart{
    pub count: Option<i64>,
    pub tahun: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Deleteparam{
    pub id: i32,
}


//Register

pub async fn register_action(state: web::Data<PoolState>, input: web::Json<RegistrasiForm>) -> impl Responder {
    let input = input.0;
    let pool = state.pool.lock().unwrap().clone();

    let test = sqlx::query_as!(RegForm,
        " SELECT email from users_fotografi where email = $1",input.email)
        .fetch_all( &pool ).await.unwrap();
    println!("doc {:?}",test);
    let user;
    if test.is_empty(){ 
        let customer = sqlx::query_as!(RegisId,
            "INSERT INTO users_fotografi (username, email,  password) VALUES ($1, $2, sha256($3)) returning id",
            input.username, 
            input.email,
            input.password.as_bytes(),
        ).fetch_one( &pool ).await.unwrap();
        user = serde_json::json!({
            "info": "Register successfully",
            "id": customer.id,
            "status": "OK",
        });
    }else{
    println!( "Eror");
    user = serde_json::json!({
        "info": "Register not successful",
        "status": "User found",
    });
    }

    // let user = serde_json::json!({
    //     "info": "Register successfully",
    //     "id": customer,
    //     "status": "OK",
    // });

    HttpResponse::Ok()
    .append_header(("Content-Type", "text/plain"))
    // .body("Data Berhasil Di-insert!")
    .json(json!(user))
}

//LOGIN

pub async fn login_action(info: web::Json<LoginForm>) -> impl Responder {
    let form = info.0;
 
    let pool = connect_postgres().await;
    let admin_id = match sqlx::query_scalar!(
        "SELECT id from users_fotografi where email = $1 and password=sha256($2);",
        form.email,
        form.password.as_bytes()
    )
    .fetch_one(&pool)
    .await
    {
        Ok(ok) => ok,
        Err(_err) => return HttpResponse::Ok().json(json!("Gagal")),
    };
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let _ = sqlx::query!(
        "UPDATE users_fotografi SET token=$1 WHERE id=$2",
        rand_string,
        admin_id
    )
    .execute(&pool)
    .await.unwrap();
    // let mut resp = Response::new(200);
    let user = serde_json::json!({
        "info": "Login successfully",
        "id": admin_id,
        "status": "OK",
        "token": rand_string,

    });

    // BUAT COOKIE 1
    let cookie = Cookie::build("pdkb-admin-ticket", rand_string)
        // .domain("")
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    HttpResponse::Ok().cookie(cookie).json(json!(user)) 
}


//LOGOUT

pub async fn logout(info: web::Query<Value>) -> impl Responder {
    let cookie = Cookie::build("gw-admin-ticket", "_")
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();
HttpResponse::Ok().cookie(cookie).json(json!("pengguna telah logout"))
}


/////Gambar 

pub async fn foto_actions(state: web::Data<PoolState>) -> impl Responder {
    let pool = state.pool.lock().unwrap().clone();
    let ft = sqlx::query_as!(Foto, "SELECT id, image, judul from hasil_foto")
        .fetch_all(&pool)
        .await
        .unwrap();

    HttpResponse::Ok().json(ft)
}

////Order

pub async fn order_action(state: web::Data<PoolState>, input: web::Json<Orders>) -> impl Responder {
    let input = input.0;
    let pool = state.pool.lock().unwrap().clone();
    let jobs = sqlx::query_as!(BlogId,
        "INSERT INTO orders (nama, pilihan_order, no_hp, jadwal_order) VALUES ($1, $2, $3::varchar::bigint, $4::varchar::date) returning id",
        input.nama,
        input.pilihan_order,
        input.no_hp,
        input.jadwal_order,
    ).fetch_one( &pool ).await.unwrap();
    let user = serde_json::json!({
        "info": "Input successfully",
        "id": jobs.id,
        "status": "OK",
    });

    HttpResponse::Ok()
    .append_header(("Content-Type", "text/plain"))
    // .body("Data Berhasil Di-insert!")
    .json(json!(user))
}

///////////////EDIT ORDER////////////////////////////////

pub async fn put_order(state: web::Data<PoolState>, input: web::Json<OrdersPut>) -> impl Responder {
    let input = input.0;
    let pool = state.pool.lock().unwrap().clone();
    // let a = input.no_hp.clone().unwrap().parse::<i64>().unwrap();
    // let b = input.jadwal_order.clone().unwrap().parse::<i64>().unwrap();
    let jobs = sqlx::query!(
        "UPDATE orders SET nama=$1,pilihan_order=$2,no_hp=$3,jadwal_order=$4 WHERE id=$5 ",
        input.nama,
        input.pilihan_order, 
        input.no_hp,
        input.jadwal_order,  
        input.id,
    ) .execute(&pool)
    .await.unwrap();
    let user = serde_json::json!({
        "info": "edit successfully", 
        "status": "OK",
    });

    HttpResponse::Ok()
    .append_header(("Content-Type", "text/plain"))
    // .body("Data Berhasil Di-insert!")
    .json(json!(user))
}



/////GRID ORDER

pub async fn getOrder_actions(state: web::Data<PoolState>) -> impl Responder {
    let pool = state.pool.lock().unwrap().clone();
    let ft = sqlx::query_as!(GetOrders, "SELECT id, nama, pilihan_order, jadwal_order::date::varchar, no_hp::varchar, status from orders")
        .fetch_all(&pool)
        .await
        .unwrap();

    HttpResponse::Ok().json(ft)
}


////Daftar Harga

pub async fn DaftarHarga(state: web::Data<PoolState>) -> impl Responder {
    let pool = state.pool.lock().unwrap().clone();
    let ft = sqlx::query_as!(Harga, "SELECT id, pilihan, pilihan_harga from harga")
        .fetch_all(&pool)
        .await
        .unwrap();

    HttpResponse::Ok().json(ft)
}


////GET CHART


pub async fn read_chart(state: web::Data<PoolState>) -> impl Responder {    
    let pool = state.pool.lock().unwrap().clone();
    let test = sqlx::query_as!(Chart,
        "SELECT count (nama), split_part(jadwal_order , '-', 1) as tahun from orders group by tahun;")
        .fetch_all( &pool )
        .await.unwrap();

    HttpResponse::Ok().json(test)
}


////////////DELETE ORDER/////////////////////////////////
// pub async fn delete_order(state: web::Data<PoolState>,input: web::Query<OrdersPut>) -> impl Responder {
//     let pool = state.pool.lock().unwrap().clone();
//     match sqlx::query!("DELETE FROM orders WHERE id=$1", input.id)
//         .fetch_one(&pool).await {
//             Ok(_x) => {println!("delete successfully")},
//             Err(e) => {println!("delete failed")}
//         }
        
//     let user = serde_json::json!({
//         "info": "delete successfully",
//         "status": "OK",
//     });

//     HttpResponse::Ok().json(user)
// }

pub async fn delete_order(state: web::Data<PoolState>,input: web::Query<Deleteparam>) -> impl Responder {
    let pool = state.pool.lock().unwrap().clone();
    match sqlx::query!("DELETE FROM orders WHERE id=$1", input.id)
        .fetch_one(&pool).await {
            Ok(_x) => {println!("delete successfully")},
            Err(e) => {println!("delete failed")}
        }
        
    let user = serde_json::json!({
        "info": "delete successfully",
        "status": "OK",
    });

    HttpResponse::Ok().json(user)
}


// pub async fn read_performa(state: web::Data<PoolState>) -> impl Responder {    
//     let pool = state.pool.lock().await.clone();
//     let test = sqlx::query_as!(Chart,
//         "select count(nama)as nama,cast(waktu as varchar) from orders group by waktu,waktu order by waktu asc;")
//         .fetch_all( &pool ).await.unwrap();

//     HttpResponse::Ok().json(test)
// }




