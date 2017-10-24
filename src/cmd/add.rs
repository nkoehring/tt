use std::io::{self, BufRead};
use persistence::save_report;
use models::Entry;

fn add_from_stdio(entries: &mut Vec<Entry>) -> Result<usize, ()> {
    let mut count = 0;

    println!("New entry (hours, comment)");
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


pub fn add_entry(mut entries: &mut Vec<Entry>, args: &[String], file_name: &str) {
    let count = if args.len() > 3 {
        add_from_args(&mut entries, &args[2..]).unwrap_or(0)
    } else {
        add_from_stdio(&mut entries).unwrap_or(0)
    };

    println!("{} entries added.", count);
    if count > 0 {
        match save_report(&entries, file_name) {
            Ok(_) => println!("{} updated", file_name),
            Err(err) => println!("Couldn't update report!\n{:?}", err),
        }
    }
}
