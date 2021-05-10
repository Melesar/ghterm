use std::fs;
use std::io::{Result, Write};

use chrono::{Local, Timelike};

pub fn start_logs() -> Result<()> {
    fs::write("log.txt", "")
}

pub fn log (message: &str) {
    let time = Local::now();
    let mut file = fs::OpenOptions::new().append(true).open("log.txt").unwrap();
    write!(file, "[{:02}:{:02}:{:02}] {} \n", time.hour(), time.minute(), time.second(), message).unwrap();
}
