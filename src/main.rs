use std::sync::Arc;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer};

use actix_web::{get, put, HttpResponse};
use rusqlite::Connection;

mod error;
mod player;
mod db;

use error::*;
use player::*;
use db::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Ok(mut conn) = Connection::open(&"./cosmetics.db") {
        setup(&mut conn)?;
        let x = web::Data::new(Arc::new(Mutex::new(conn)));
        HttpServer::new(move || App::new().service(get).service(put).service(put_costume).service(put_slot).app_data(x.clone()))
            .bind("127.0.0.1:8080")?
            .run()
            .await?;
    }
    Ok(())
}


#[get("/{names}")]
pub async fn get(
    web::Path(names): web::Path<String>,
    conn: web::Data<Arc<Mutex<Connection>>>,
) -> Result<HttpResponse, Error> {
    let l = conn.lock().unwrap();
    Ok(HttpResponse::Ok().json(get_players(&l, names)?))
}

#[put("/player/{token}")]
pub async fn put(
    web::Path(token): web::Path<String>,
    p: web::Json<Player>,
    conn: web::Data<Arc<Mutex<Connection>>>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", p);
    println!("{}", token);
    let l = conn.lock().unwrap();
    Ok(HttpResponse::Ok().json(set_player(&l, p.into_inner(), token)?))
}

#[put("/costume/{token}/{slot}")]
pub async fn put_costume(
    web::Path(token): web::Path<String>,
    p: web::Json<Player>,
    slot: web::Path<i32>,
    conn: web::Data<Arc<Mutex<Connection>>>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", p);
    println!("{}", token);
    let l = conn.lock().unwrap();
    Ok(HttpResponse::Ok().json(set_costume(&l, p.into_inner(), *slot, token)?))
}

#[put("/slot/{token}/{slot}")]
pub async fn put_slot(
    web::Path(token): web::Path<String>,
    name: web::Path<String>,
    slot: web::Path<i32>,
    conn: web::Data<Arc<Mutex<Connection>>>,
) -> Result<HttpResponse, Error> {
    println!("{}", token);
    let l = conn.lock().unwrap();
    Ok(HttpResponse::Ok().json(set_slot(&l, (*name).clone(), *slot, token)?))
}