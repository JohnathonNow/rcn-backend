use std::sync::Arc;
use std::sync::Mutex;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use rusqlite::Connection;

use std::sync::Arc;
use std::sync::Mutex;

use crate::error::Error;
use actix_web::{get, put, web, HttpResponse};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

mod error;
mod games;
mod guesses;
mod players;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Ok(mut conn) = Connection::open(&"./cosmetics.db") {
        guesses::setup(&mut conn)?;
        games::setup(&mut conn)?;
        let x = web::Data::new(Arc::new(Mutex::new(conn)));
        HttpServer::new(move || {
            App::new()
                .service(get)
                .service(put)
                .app_data(x.clone())
        })
        .bind("127.0.0.1:8080")?
            .run()
            .await?;
    }
    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
struct Player {
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
                shld              INT,
              )",
              params![],
              )
        .unwrap();
    Ok(())
}

#[get("/")]
pub async fn get(
    web::Path(game_id): web::Path<i64>,
    conn: web::Data<Arc<Mutex<Connection>>>,
    ) -> Result<HttpResponse, Error> {
    let l = conn.lock().unwrap();
    let mut stmt = l.prepare("SELECT * FROM players WHERE name IN ?1")?;
    let iter = stmt
        .query_map(params![game_id], |row| {
            Ok(Player {
                id: row.get(0)?,
                name: row.get(0)?,
                head: row.get(2)?,
                body: row.get(3)?,
                cape: row.get(4)?,
                legs: row.get(5)?,
                neck: row.get(6)?,
                hand: row.get(7)?,
                ring: row.get(8)?,
                feet: row.get(9)?,
                weap: row.get(10)?,
                shld: row.get(11)?,
            })
        })?
    .filter(|a| a.is_ok())
        .map(|x| x.unwrap());
    Ok(HttpResponse::Ok().json(iter.collect::<Vec<Guess>>()))
}

#[put("/")]
pub async fn put(
    web::Path((game_id, user, text)): web::Path<(i64, String, String)>,
    conn: web::Data<Arc<Mutex<Connection>>>,
    ) -> Result<HttpResponse, Error> {
    let r = conn.lock().unwrap().execute(
        "INSERT INTO guesses (game_id, user, text) VALUES (?1, ?2, ?3);
             UPDATE games SET version = version + 1 WHERE gameid = ?1;",
             params![game_id, user, text],
             )?;
    Ok(HttpResponse::Ok().json(r))
}
