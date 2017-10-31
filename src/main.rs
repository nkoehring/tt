extern crate chrono;
extern crate colored;
extern crate xdg;
extern crate toml;
use std::env::args;
use std::path::PathBuf;

mod cmd;
mod models;
mod persistence;
mod tomlbox;
use models::Entry;
use persistence::load_report;


fn main() {
    let commands = ["report", "all", "add"];
    let args: Vec<String> = args().skip(1).collect();

    let (project_name, command, command_args): (Option<&str>, &str, _) =
        match args.iter().position(|x| commands.contains(&x.as_str())) {
            Some(0) => (None, &args[0], &args[1..]),
            Some(1) => (Some(&args[0]), &args[1], &args[2..]),
            _       => (
                if args.len() > 0 { Some(&args[0]) } else { None },
                "report",
                &args
            ),
        };

    let mut entries: Vec<Entry> = vec![];

    let config = persistence::load_config();
    let project = config.project(project_name);

    if let (None, Some(name)) = (project, project_name) {
       println!("Project “{}” is not known to me, sorry.", name);
        ::std::process::exit(1);
    }

    let project_path = match project {
        Some(project) => {
            println!("Loaded report from {}", &project.path.display());
            PathBuf::from(&project.path)
        },
        None => {
            println!("No (existing/default) projects configured.");
            println!("Trying to load local ./report.csv.");
            PathBuf::from("./report.csv")
        }
    };

    if let Err(_) = load_report(&mut entries, &project_path) {
        println!("Couldn't load any report. Is your project configuration correct?");
        println!("Consider adding a project to your config file (usually ~/.config/tt/config.toml),");
        println!("or creating a file right here with `touch ./report.csv`");
        ::std::process::exit(1);
    }

    match command.as_ref() {
        "report" => cmd::condensed_report(&entries, project),
        "all" => cmd::verbose_report(&entries, project),
        "add" => cmd::add_entry(&mut entries, &command_args, &project_path),
        _ => println!("Usage: tt report|add"),
    }

}
