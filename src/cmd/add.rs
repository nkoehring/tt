use std::io::{self, BufRead};
use std::path::PathBuf;
use persistence::save_report;
use models::Entry;
use colored::*;

fn add_from_stdio(entries: &mut Vec<Entry>) -> Result<usize, ()> {
    let mut count = 0;

    let msg1 = "Add entries line by line like ‘".italic();
    let msg2 = "2.5 foo bar baz".bold();
    let msg3 = "’ and press CTRL+d when finished".italic();
    println!("{}{}{}", msg1, msg2, msg3);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if let Some(entry) = Entry::from(&line) {
            entries.push(entry);
            count += 1;
        }
    }

    Ok(count)
}


fn add_from_args(entries: &mut Vec<Entry>, slice: &[String]) -> Result<usize, ()> {
    if let Some(entry) = Entry::from(&slice.join(" ")) {
        entries.push(entry);
        Ok(1)
    } else {
        Err(())
    }
}


pub fn add_entry(mut entries: &mut Vec<Entry>, args: &[String], file_name: &PathBuf) {
    let count = if args.len() > 0 {
        add_from_args(&mut entries, args).unwrap_or(0)
    } else {
        add_from_stdio(&mut entries).unwrap_or(0)
    };

    println!("{} entries added", count);
    if count > 0 {
        match save_report(&entries, file_name) {
            Ok(_) => println!("{} updated", file_name.display()),
            Err(err) => println!("Couldn't update report!\n{:?}", err),
        }
    }
}
