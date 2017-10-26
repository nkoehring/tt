use std::io::{self, Read, BufReader};
use std::fs::File;
use std::path::PathBuf;
use xdg::BaseDirectories;
use toml;
use toml::value::{Table, Value};

use models::{Config, ProjectCfg};


fn check_path() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("tt")
                   .expect("XDG doesn't work and I don't know why :S");

    xdg_dirs.place_config_file("config.toml")
    .expect("Unable to create configuration path :(")
}


fn load_toml(path: &PathBuf) -> io::Result<Table> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut raw_data = String::new();

    let _bytes_read = reader.read_to_string(&mut raw_data)?;
    match toml::from_str::<Table>(&raw_data) {
        Ok(data) => Ok(data),
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "toml parser error"))
    }
}


fn cfg_string(key: &str, table: &Table) -> Option<String> {
    table.get(key).and_then(|x| x.as_str()).map(String::from)
}

fn cfg_number(key: &str, table: &Table) -> Option<f64> {
    match table.get(key) {
        Some(&Value::Float(f)) => Some(f),
        Some(&Value::Integer(i)) => Some(i as f64),
        _ => None,
    }
}

fn cfg_path(key: &str, table: &Table) -> Option<PathBuf> {
    table.get(key).and_then(|x| x.as_str()).map(PathBuf::from)
}


fn parse_config(cfg: &Table) -> Config {
    let default_project = cfg_string("default_project", cfg);
    let default_currency = cfg_string("default_currency", cfg);
    let default_rate = cfg_number("default_rate", cfg);

    let default_keys = ["default_project", "default_currency", "default_rate"];
    let mut projects = vec![];

    for (key, value) in cfg {
        if default_keys.contains(&key.as_ref()) { continue; }

        let value = value.as_table().expect("Project entries need subkeys");
        let project = ProjectCfg {
            name: key.clone(),
            path: cfg_path("path", value).expect("Project path is obligatory!"),
            currency: cfg_string("currency", value).or(default_currency.clone()),
            rate: cfg_number("rate", value).or(default_rate.clone()),
        };
        projects.push(project);
    }

    Config {
        default_project: default_project,
        default_currency: default_currency,
        default_rate: default_rate,
        projects: projects,
    }
}


pub fn load() -> Config {
    let config_path = check_path();
    match load_toml(&config_path) {
        Ok(raw_config) => parse_config(&raw_config),
        Err(err) => panic!("{:?}", err),
    }
}
