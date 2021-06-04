use std::{thread, time::{SystemTime, UNIX_EPOCH}};
fn request() {
    reqwest::blocking::get("http://localhost:8080/aaaaaaaaa,aaaaaaaaaaa,aaaaaaaaa,aaaaaaaaaa,aaaaaaaaaaaaa,aaaaaaaaaaaa,aaaaaaaaaaaaaaaa,TheArmy,aaaaaaaaaaa,aaaaaaaaa,aaaaaaaaaaaaaa,aaaaaaaaa").ok();
}
fn threadyboi() {
    loop {
        let now = SystemTime::now();
        let start = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        request();
        let now = SystemTime::now();
        let end = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        println!("{}", end - start);
        //println!("{}", end);
    }
}
pub fn main() {
    for _ in 0..10000 {
        thread::spawn(threadyboi);
    }
    loop {}
}
