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
        "CREATE TABLE IF NOT EXISTS costumes (
                id                INTEGER PRIMARY KEY,
                name              TEXT,
                slot              INTEGER NOT NULL,
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
                timestamp         INTEGER,
                UNIQUE(name, slot)
              );",
        params![]).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tokens (
                id                INTEGER PRIMARY KEY,
                token             TEXT NOT NULL,
                name              TEXT
              );",
        params![]).unwrap();
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
        params![]).unwrap();
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

pub fn set_player(conn: &Connection, p: Player, token: String) -> Result<usize, Error> {
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::new(0, 0))
        .as_secs();
    let r = conn.execute(
        "INSERT INTO players (name, head, body, cape, legs, neck, hand, ring, feet, weap, shld, jaws, hair, token_id, timestamp)
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

pub fn set_costume(conn: &Connection, p: Player, slot: i32, token: String) -> Result<usize, Error> {
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::new(0, 0))
        .as_secs();
    let r = conn.execute(
        "INSERT INTO costumes (name, head, body, cape, legs, neck, hand, ring, feet, weap, shld, jaws, hair, token_id, timestamp, slot)
         VALUES
         (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 
            (SELECT id FROM tokens WHERE token=?14), ?15, ?16)
        ON CONFLICT(name, slot) DO UPDATE SET 
            name=?1,head=?2,body=?3,cape=?4,legs=?5,neck=?6,hand=?7,ring=?8,feet=?9,weap=?10,shld=?11,jaws=?12,hair=?13,
                token_id=(SELECT id FROM tokens WHERE token=?14),timestamp=?15,slot=?16;",
             params![p.name, p.head, p.body, p.cape, p.legs, p.neck, p.hand, p.ring, p.feet, p.weap, p.shld, p.jaws, p.hair, token, ts, slot],
             )?;
    Ok(r)
}

pub fn set_slot(conn: &Connection, name: String, slot: i32, token: String) -> Result<usize, Error> {
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::new(0, 0))
        .as_secs();
    let r = conn.execute(
        "REPLACE INTO players (name, head, body, cape, legs, neck, hand, ring, feet, weap, shld, jaws, hair, token_id, timestamp)
        SELECT name, head, body, cape, legs, neck, hand, ring, feet, weap, shld, jaws, hair, (SELECT id FROM tokens WHERE token=?3) as token_id, ?4 as timestamp
        FROM costumes WHERE name=?1 AND slot=?2;",
             params![name, slot, token, ts],
             )?;
    Ok(r)
}

#[cfg(test)]
mod test {
    use super::*;
    fn player_generic(name: String) -> Player {
        Player {
            name,
            head: 1,
            body: 2,
            cape: 3,
            legs: 4,
            neck: 5,
            hand: 6,
            ring: 7,
            feet: 8,
            weap: 9,
            shld: 10,
            jaws: 11,
            hair: 12,
        }
    }

    fn add_token(conn: &Connection, token: String, name: String) -> Result<usize, Error> {
        let r = conn.execute(
            "INSERT INTO tokens(name, token) VALUES (?1, ?2);",
            params![name, token],
        )?;
        Ok(r)
    }

    #[test]
    fn test_setup() {
        let conn = Connection::open_in_memory().unwrap();
        setup(&conn).unwrap();
    }

    #[test]
    fn test_set_players() {
        let conn = Connection::open_in_memory().unwrap();
        setup(&conn).unwrap();
        assert!(set_player(&conn, player_generic("Jeff".into()), "invalid_token".into()).is_err());
        assert_eq!(
            add_token(&conn, "12345".into(), "Jeffrey".into()).unwrap(),
            1
        );
        assert_eq!(
            set_player(&conn, player_generic("Jeff".into()), "12345".into()).unwrap(),
            1
        );
    }

    #[test]
    fn test_get_players() {
        let conn = Connection::open_in_memory().unwrap();
        setup(&conn).unwrap();
        add_token(&conn, "12345".into(), "Jeffrey".into()).unwrap();
        set_player(&conn, player_generic("Jeff1".into()), "12345".into()).unwrap();
        set_player(&conn, player_generic("Jeff2".into()), "12345".into()).unwrap();
        set_player(&conn, player_generic("Jeff3".into()), "12345".into()).unwrap();
        assert!(get_players(&conn, "".into()).unwrap().is_empty());
        assert!(get_players(&conn, "NotAValidName".into()).unwrap().is_empty());
        let v = get_players(&conn, "Jeff1".into()).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].name, "Jeff1");
        let v = get_players(&conn, "Jeff2,Jeff3".into()).unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].name, "Jeff2");
        assert_eq!(v[1].name, "Jeff3");
    }

    #[test]
    fn test_set_costumes() {
        let conn = Connection::open_in_memory().unwrap();
        setup(&conn).unwrap();
        assert!(set_costume(&conn, player_generic("Jeff".into()), 0, "invalid_token".into()).is_err());
        assert_eq!(
            add_token(&conn, "12345".into(), "Jeffrey".into()).unwrap(),
            1
        );
        assert_eq!(
            set_costume(&conn, player_generic("Jeff".into()), 0, "12345".into()).unwrap(),
            1
        );
    }

    #[test]
    fn test_set_slot() {
        let conn = Connection::open_in_memory().unwrap();
        setup(&conn).unwrap();
        assert_eq!(
            add_token(&conn, "12345".into(), "Jeffrey".into()).unwrap(),
            1
        );
        assert_eq!(
            set_costume(&conn, player_generic("Jeff".into()), 0, "12345".into()).unwrap(),
            1
        );
        assert!(set_slot(&conn, "Jeff".into(), 0, "invalid_token".into()).is_err());
        assert_eq!(
            set_slot(&conn, "Jeff".into(), 0, "12345".into()).unwrap(),
            1
        );
    }

    #[test]
    fn test_costumes_full() {
        let conn = Connection::open_in_memory().unwrap();
        setup(&conn).unwrap();
        let mut p = player_generic("Jeff".into());
        add_token(&conn, "12345".into(), "Jeffrey".into()).unwrap();
        set_player(&conn, p.clone(), "12345".into()).unwrap();
        p.head = 2607;
        assert_eq!(
            set_costume(&conn, p.clone(), 0, "12345".into()).unwrap(),
            1
        );
        let v = &get_players(&conn, "Jeff".into()).unwrap()[0];
        assert_eq!(v.head, 1);
        assert_eq!(p.name, v.name);
        assert_eq!(p.body, v.body);
        assert_eq!(
            set_slot(&conn, "Jeff".into(), 0, "12345".into()).unwrap(),
            1
        );
        let v = &get_players(&conn, "Jeff".into()).unwrap()[0];
        assert_eq!(v.head, 2607);
        assert_eq!(p.name, v.name);
        assert_eq!(p.body, v.body);
    }
}
