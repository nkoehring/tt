extern crate chrono;
extern crate colored;
extern crate xdg;
extern crate toml;
use std::env::args;
use std::path::PathBuf;

mod cmd;
mod models;
mod persistence;
mod configuration;
use models::Entry;
use persistence::load_report;


fn main() {
    let args: Vec<String> = args().collect();
    let cmd = if args.len() > 1 { args[1].clone() } else { "report".to_owned() };
    let mut entries: Vec<Entry> = vec![];

    let config = configuration::load();
    println!("Configuration:\n{:#?}\n", config);

    let project_path = match config.project() {
        Some(project) => PathBuf::from(&project.path),
        None => PathBuf::from("./report.csv")
    };

    match load_report(&mut entries, &project_path) {
        Ok(c) => println!("Loaded {} awesome entries!", c),
        Err(_) => {
            println!("Couldn't load any report. I guess you just start?");
            ::std::process::exit(1);
        }
    }

    match cmd.as_ref() {
        "report" => cmd::condensed_report(&entries),
        "all" => cmd::verbose_report(&entries),
        "add" => cmd::add_entry(&mut entries, &args, &project_path),
        _ => println!("Usage: tt report|add"),
    }

}
