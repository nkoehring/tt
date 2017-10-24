extern crate chrono;
extern crate colored;
use std::env::args;

mod cmd;
mod models;
mod persistence;
use models::Entry;
use persistence::load_report;


fn main() {
    let file_name = "report.csv";
    let args: Vec<String> = args().collect();
    let cmd = if args.len() > 1 { args[1].clone() } else { "report".to_owned() };
    let mut entries: Vec<Entry> = vec![];

    match load_report(&mut entries, file_name) {
        Ok(c) => println!("Loaded {} awesome entries!", c),
        Err(_) => {
            println!("Couldn't load any report. I guess you just start?");
            ::std::process::exit(1);
        }
    }

    match cmd.as_ref() {
        "report" => cmd::condensed_report(&entries),
        "all" => cmd::verbose_report(&entries),
        "add" => cmd::add_entry(&mut entries, &args, file_name),
        _ => println!("Usage: tt report|add"),
    }

}
