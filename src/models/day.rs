use std::fmt;
use colored::*;


pub struct Day {
    pub date: String,
    pub hours: f64,
    pub comments: Vec<String>,
}

impl Day {
    fn dedup_comments(&self) -> String {
        let mut output = self.comments[0].clone();

        for c in self.comments[1..].iter() {
            if let None = output.find(c) {
                output += &format!(", {}", &c);
            }
        }

        output
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = &format!("{:4.1}h", self.hours);
        write!(f, "{}: {} {}", self.date, hours.bold(), self.dedup_comments().italic())
    }
}
