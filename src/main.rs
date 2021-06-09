use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};
use actix_web::{web, App, HttpServer};
use rusqlite::Connection;

use actix_web::{get, put, HttpResponse};

use rusqlite::params;
use rusqlite::types::Value;
use rusqlite::vtab::array;

mod error;
mod player;
use error::*;
use player::*;

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

pub fn setup(conn: &mut Connection) -> std::io::Result<()> {
    array::load_module(&conn).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
                name              TEXT PRIMARY KEY,
                head              INTEGER,
                body              INTEGER,
                cape              INTEGER,
                legs              INTEGER,
                neck              INTEGER,
                hand              INTEGER,
                ring              INTEGER,
                feet              INTEGER,
                weap              INTEGER,
                shld              INTEGER,
                jaws              INTEGER,
                hair              INTEGER,
                token_id          INTEGER NOT NULL,
                timestamp         INTEGER
              );",
        params![],
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tokens (
                id                INTEGER PRIMARY KEY,
                token             TEXT NOT NULL,
                name              TEXT
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
    let names: Vec<Value> = names
        .split(",")
        .map(|i| Value::from(i.to_string()))
        .collect();
    let ptr = Rc::new(names);
    let mut stmt = l.prepare("SELECT * FROM players WHERE name IN rarray(?);")?;
    let iter = stmt
        .query_map(&[&ptr], |row| Ok(row.into()))?
        .filter(|a| a.is_ok())
        .map(|x| x.unwrap());
    Ok(HttpResponse::Ok().json(iter.collect::<Vec<Player>>()))
}

#[put("/{token}")]
pub async fn put(
    web::Path(token): web::Path<String>,
    p: web::Json<Player>,
    conn: web::Data<Arc<Mutex<Connection>>>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", p);
    println!("{}", token);
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::new(0, 0)).as_secs();
    let r = conn.lock().unwrap().execute(
        "INSERT INTO players (name, head, body, cape, legs, neck, hand, ring, feet, weap, shld, jaws, hair, token_id)
         VALUES
         (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 
            (SELECT id FROM tokens WHERE token=?14), ?15)
        ON CONFLICT(name) DO UPDATE SET 
            name=?1,head=?2,body=?3,cape=?4,legs=?5,neck=?6,hand=?7,ring=?8,feet=?9,weap=?10,shld=?11,jaws=?12,hair=?13,
                token_id=(SELECT id FROM tokens WHERE token=?14),timestamp=?15;",
             params![p.name, p.head, p.body, p.cape, p.legs, p.neck, p.hand, p.ring, p.feet, p.weap, p.shld, p.jaws, p.hair, token, ts],
             )?;
    Ok(HttpResponse::Ok().json(r))
}
