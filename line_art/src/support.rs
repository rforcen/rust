#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;


// returns the string
pub fn read_file(path: &str) -> Result<String, &str> {
    if let Ok(mut file) = File::open(path) {
        let mut buff = String::new();
        if let Ok(_size) = file.read_to_string(&mut buff) {
            Ok(buff)
        } else {
            Err("read error")
        }
    } else {
        Err("file not found")
    }
}
