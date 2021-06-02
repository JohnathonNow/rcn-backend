use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use regex::Regex;
use rusqlite::Connection;

use actix_web::{get, put, HttpResponse};
use actix_web::{http::StatusCode, ResponseError};
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Error {
    s: String,
}

impl Error {
    pub fn new(s: &str) -> Self {
        Self { s: s.into() }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("bob")
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Self { s: e.to_string() }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        self.s.clone().into()
    }
}
lazy_static! {
    static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9_]*$").unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Ok(mut conn) = Connection::open(&"./cosmetics.db") {
        setup(&mut conn)?;
        let x = web::Data::new(Arc::new(Mutex::new(conn)));
        HttpServer::new(move || App::new().service(get).service(put).app_data(x.clone()))
            .bind("127.0.0.1:8080")?
            .run()
            .await?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
    head: i32,
    body: i32,
    cape: i32,
    legs: i32,
    neck: i32,
    hand: i32,
    ring: i32,
    feet: i32,
    weap: i32,
    shld: i32,
}

pub fn setup(conn: &mut Connection) -> std::io::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
                name              TEXT PRIMARY KEY,
                head              INT,
                body              INT,
                cape              INT,
                legs              INT,
                neck              INT,
                hand              INT,
                ring              INT,
                feet              INT,
                weap              INT,
                shld              INT
              );",
        params![],
    )
    .unwrap();
    Ok(())
}

#[get("/{names}")]
pub async fn get(
    web::Path(names): web::Path<String>,
    conn: web::Data<Arc<Mutex<Connection>>>,
) -> Result<HttpResponse, Error> {
    let l = conn.lock().unwrap();
    let names_sane: Vec<String> = names
        .split(",")
        .filter(|x| RE.is_match(x))
        .map(|x| format!("\"{}\"", x))
        .collect();
    println!("{:?}", names_sane);
    let s = format!(
        "SELECT * FROM players WHERE name IN ({});",
        names_sane.join(",")
    );
    let mut stmt = l.prepare(&s)?;
    let iter = stmt
        .query_map(params![], |row| {
            Ok(Player {
                name: row.get(0)?,
                head: row.get(1)?,
                body: row.get(2)?,
                cape: row.get(3)?,
                legs: row.get(4)?,
                neck: row.get(5)?,
                hand: row.get(6)?,
                ring: row.get(7)?,
                feet: row.get(8)?,
                weap: row.get(9)?,
                shld: row.get(10)?,
            })
        })?
        .filter(|a| a.is_ok())
        .map(|x| x.unwrap());
    Ok(HttpResponse::Ok().json(iter.collect::<Vec<Player>>()))
}

#[put("/")]
pub async fn put(
    player: web::Json<Player>,
    conn: web::Data<Arc<Mutex<Connection>>>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", player);
    let r = conn.lock().unwrap().execute(
        "INSERT INTO players (name, head, body, cape, legs, neck, hand, ring, feet, weap, shld) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11) ON CONFLICT(name) DO UPDATE SET name=?1,head=?2,body=?3,cape=?4,legs=?5,neck=?6,hand=?7,ring=?8,feet=?9,weap=?10,shld=?11;",
             params![player.name, player.head, player.body, player.cape, player.legs, player.neck, player.hand, player.ring, player.feet, player.weap, player.shld],
             )?;
    Ok(HttpResponse::Ok().json(r))
}
