use models::{Day, Entry, ProjectCfg};


fn summary(total_hours: f64, project: Option<&ProjectCfg>) {
    if let Some(project) = project {
        println!("\nProject {}: {:.1} hours total.", project.name, total_hours);

        if let Some(rate) = project.rate {
            let currency = match project.currency {
                Some(ref currency) => currency,
                _ => "€"
            };
            println!("Total earnings are {}{:.2}", currency, total_hours * rate);
        }
    } else {
        println!("Total {:.1} hours.", total_hours);
    }
}


pub fn verbose_report(entries: &[Entry], project: Option<&ProjectCfg>) {
    let mut total_hours = 0.0;

    for entry in entries {
        total_hours += entry.hours;
        println!("{}", entry);
    }

    summary(total_hours, project);
}


pub fn condensed_report(entries: &[Entry], project: Option<&ProjectCfg>) {
    if entries.len() == 0 {
        return summary(0.0, project);
    }

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

    summary(hours, project);
}
