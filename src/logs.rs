use std::{fs, io::Write};

use chrono::{Local, Timelike};

pub fn log (message: &str) {
    let time = Local::now();
    let mut file = fs::OpenOptions::new().append(true).create(true).open("log.txt").unwrap();
    write!(file, "[{:02}:{:02}:{:02}] {} \n", time.hour(), time.minute(), time.second(), message).unwrap();
}

pub fn clear_logs() {
    fs::remove_file("log.txt").unwrap();
}
