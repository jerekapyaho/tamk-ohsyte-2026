use std::fs;
use std::path::PathBuf;

use chrono::{Datelike, Local, NaiveDate};
use clap::{Parser, Subcommand};
use dirs;

use today::events::{Category, Event, MonthDay};
use today::filters::FilterBuilder;
use today::{Config, run};

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// List all event providers
    Providers,
}

#[derive(Parser)]
#[command(name = "today")]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,

    #[arg(short, long, help = "Event date in MMDD format")]
    date: Option<String>,
}

fn main() {
    let args = Args::parse();

    let month_day = if let Some(md) = args.date {
        MonthDay::from_str(&md)
    } else {
        let today: NaiveDate = Local::now().date_naive();
        MonthDay::new(today.month(), today.day())
    };

    let filter = FilterBuilder::new().month_day(month_day).build();

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            println!("Looking for configuration file '{}'", &toml_path.display());
            let config_str = fs::read_to_string(toml_path).expect("existing configuration file");
            let config: Config = toml::from_str(&config_str).expect("valid configuration file");
            //println!("config: {:#?}", config);

            match args.cmd {
                Some(Command::Providers) => {
                    for provider in config.providers {
                        println!("{}\t{}", provider.name, provider.kind);
                    }
                },
                None => {
                    if let Err(e) = today::run(&config, &path, &filter) {
                        eprintln!("Error running program: {}", e);
                        return;
                    }
                }
            }
        }
        None => {
            eprintln!("Unable to configure the application");
            return;
        }
    }
}

// Gets the configuration directory path for the application
// named in the `app_name` argument.
// If the directory does not exist, tries to create it.
// Returns an optional `PathBuf` containing the directory path,
// or None if the directory can't be created.
fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        println!("Config directory: '{}'", config_dir.display());

        // Check if our config directory exists
        let config_path = config_dir.join(app_name);
        print!("App config directory: '{}'", config_path.display());

        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            } else {
                print!(" - created");
            }
        } else {
            print!(" - exists");
        }
        println!();

        return Some(config_path);
    }

    None
}
