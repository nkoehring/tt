use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::path::PathBuf;
use xdg::BaseDirectories;

use models::{Entry, Config};
use tomlbox::*;


pub fn load_report(entries: &mut Vec<Entry>, path: &PathBuf) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut count = 0;

    for line in BufReader::new(file).lines() {
        let entry = Entry::deserialize(&line?);
        entries.push(entry);
        count += 1;
    }

    // sort entries by date, trivial thanks to the date format YYYY-MM-DD
    entries.sort_by(|a,b| a.date.cmp(&b.date));

    Ok(count)
}


pub fn save_report(entries: &[Entry], path: &PathBuf) -> Result<usize, io::Error> {
    let mut file = File::create(path)?;
    let map = entries.into_iter().map(|e| format!("{:?}", e));
    let output = map.collect::<Vec<String>>().join("\n");

    file.write(output.as_bytes())
}


fn check_config_path() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("tt")
                   .expect("XDG doesn't work and I don't know why :S");

    xdg_dirs.place_config_file("config.toml")
    .expect("Unable to create configuration path :(")
}


pub fn load_config() -> Config {
    let config_path = check_config_path();
    match load_toml(&config_path) {
        Ok(raw_config) => Config::from_toml(&raw_config),
        Err(err) => panic!("{:?}", err),
    }
}
