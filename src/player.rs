use rusqlite::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub head: i32,
    pub body: i32,
    pub cape: i32,
    pub legs: i32,
    pub neck: i32,
    pub hand: i32,
    pub ring: i32,
    pub feet: i32,
    pub weap: i32,
    pub shld: i32,
    pub jaws: i32,
    pub hair: i32,
}

impl From<&Row<'_>> for Player {
    fn from(row: &Row) -> Self {
        Player {
            name: row.get(0).unwrap_or(String::new()),
            head: row.get(1).unwrap_or(0),
            body: row.get(2).unwrap_or(0),
            cape: row.get(3).unwrap_or(0),
            legs: row.get(4).unwrap_or(0),
            neck: row.get(5).unwrap_or(0),
            hand: row.get(6).unwrap_or(0),
            ring: row.get(7).unwrap_or(0),
            feet: row.get(8).unwrap_or(0),
            weap: row.get(9).unwrap_or(0),
            shld: row.get(10).unwrap_or(0),
            jaws: row.get(11).unwrap_or(0),
            hair: row.get(12).unwrap_or(0),
        }
    }
}