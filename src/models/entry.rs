use std::fmt;
use chrono::Local;
use colored::*;


pub struct Entry {
    pub date: String,
    pub hours: f64,
    pub comment: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = &format!("{:4.1}h", self.hours);
        write!(f, "{}: {} {}", self.date, hours.bold(), self.comment.italic())
    }
}
impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:.1},\"{}\"", self.date, self.hours, self.comment)
    }
}

impl Entry {
    pub fn new(hours: f64, comment: &str) -> Entry {
        let date = Local::today().format("%F");
        Entry {
            date: date.to_string(),
            hours: hours,
            comment: comment.to_owned(),
        }
    }

    pub fn from(line: &str) -> Option<Entry> {
        match line.find(char::is_whitespace) {
            Some(idx) => {
                let hours = line[..idx].parse::<f64>().unwrap_or_default();
                let entry = Entry::new(hours, line[idx..].trim());
                Some(entry)
            },
            None => None,
        }
    }

    pub fn deserialize(line: &str) -> Entry {
        let items: Vec<&str> = line.split(',').collect();
        let date = items[0];
        let hours = items[1];
        let comment = items[2..].join("").trim_matches('"').to_owned();

        Entry {
            date: date.to_string(),
            hours: hours.parse::<f64>().unwrap_or(0.0),
            comment: comment,
        }
    }
}
