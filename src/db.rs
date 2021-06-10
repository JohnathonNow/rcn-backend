use rusqlite::params;
use rusqlite::types::Value;
use rusqlite::vtab::array;
use rusqlite::Connection;

use std::rc::Rc;
use std::time::{Duration, SystemTime};

use crate::error::*;
use crate::player::*;

pub fn setup(conn: &Connection) -> std::io::Result<()> {
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

pub fn get_players(conn: &Connection, names: String) -> Result<Vec<Player>, Error> {
    let names: Vec<Value> = names
        .split(",")
        .map(|i| Value::from(i.to_string()))
        .collect();
    let ptr = Rc::new(names);
    let mut stmt = conn.prepare("SELECT * FROM players WHERE name IN rarray(?);")?;
    let iter = stmt
        .query_map(&[&ptr], |row| Ok(row.into()))?
        .filter(|a| a.is_ok())
        .map(|x| x.unwrap());
        Ok(iter.collect::<Vec<Player>>())
}

pub fn set_player(conn: &Connection, p: Player, token: String) -> Result<usize, Error>{
        let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::new(0, 0)).as_secs();
    let r = conn.execute(
        "INSERT INTO players (name, head, body, cape, legs, neck, hand, ring, feet, weap, shld, jaws, hair, token_id)
         VALUES
         (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 
            (SELECT id FROM tokens WHERE token=?14), ?15)
        ON CONFLICT(name) DO UPDATE SET 
            name=?1,head=?2,body=?3,cape=?4,legs=?5,neck=?6,hand=?7,ring=?8,feet=?9,weap=?10,shld=?11,jaws=?12,hair=?13,
                token_id=(SELECT id FROM tokens WHERE token=?14),timestamp=?15;",
             params![p.name, p.head, p.body, p.cape, p.legs, p.neck, p.hand, p.ring, p.feet, p.weap, p.shld, p.jaws, p.hair, token, ts],
             )?;
        Ok(r)
}