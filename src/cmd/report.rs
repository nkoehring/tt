use models::{Day, Entry};

pub fn verbose_report(entries: &[Entry]) {
    for entry in entries {
        println!("{}", entry);
    }
}


pub fn condensed_report(entries: &[Entry]) {
    let last_index = entries.len() - 1;
    let mut days = vec![];
    let mut cursor = &entries[0].date;

    let mut hours = 0.0;
    let mut comments = vec![];

    for (i, entry) in entries.iter().enumerate() {
        if &entry.date == cursor {
            hours += entry.hours;
            comments.push(entry.comment.clone());
        } else {
            let day = Day { date: cursor.to_owned(), hours: hours, comments: comments.clone() };
            days.push(day);
            hours = entry.hours;
            comments = vec![entry.comment.clone()];
            cursor = &entry.date;
        }
        if i >= last_index {
            let day = Day { date: entry.date.clone(), hours: hours, comments: comments.clone() };
            days.push(day);
        }
    }

    hours = 0.0;
    for day in days {
        hours += day.hours;
        println!("{}", day);
    }
}
