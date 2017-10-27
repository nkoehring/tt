use std::io::{self, Read, BufReader};
use std::fs::File;
use std::path::PathBuf;

use toml;
use toml::value::{Table, Value};


pub fn cfg_string(key: &str, table: &Table) -> Option<String> {
    table.get(key).and_then(|x| x.as_str()).map(String::from)
}

pub fn cfg_number(key: &str, table: &Table) -> Option<f64> {
    match table.get(key) {
        Some(&Value::Float(f)) => Some(f),
        Some(&Value::Integer(i)) => Some(i as f64),
        _ => None,
    }
}

pub fn cfg_path(key: &str, table: &Table) -> Option<PathBuf> {
    table.get(key).and_then(|x| x.as_str()).map(PathBuf::from)
}

pub fn load_toml(path: &PathBuf) -> io::Result<Table> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut raw_data = String::new();

    let _bytes_read = reader.read_to_string(&mut raw_data)?;
    match toml::from_str::<Table>(&raw_data) {
        Ok(data) => Ok(data),
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "toml parser error"))
    }
}


