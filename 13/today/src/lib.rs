use std::error::Error;
use std::path::Path;

pub mod birthday;
pub mod events;
pub mod filters;
pub mod providers;

use birthday::handle_birthday;
use chrono::{Datelike, Local, NaiveDate};
use serde::Deserialize;

use crate::events::EventKind;
use crate::filters::EventFilter;
use crate::providers::{
    csvfile::CSVFileProvider, sqlite::SQLiteProvider, textfile::TextFileProvider, web::WebProvider,
};
use events::{Category, Event, MonthDay};
use providers::{EventProvider, SimpleProvider};

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
}

pub fn create_providers(config: &Config, config_path: &Path) -> Vec<Box<dyn EventProvider>> {
    // Try to create all the event providers specified in `config`.
    // Put them in a vector of trait objects.
    let mut providers: Vec<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        let path = config_path.join(&cfg.resource);
        match cfg.kind.as_str() {
            "text" => {
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            }
            "csv" => {
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            }
            "sqlite" => {
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            }
            "web" => {
                let provider = WebProvider::new(&cfg.name, &cfg.resource);
                providers.push(Box::new(provider));
            }
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }

    let test_provider = SimpleProvider::new("test");
    providers.push(Box::new(test_provider));

    providers
}

pub fn run(
    config: &Config,
    config_path: &Path,
    filter: &EventFilter,
) -> Result<(), Box<dyn Error>> {
    handle_birthday();

    let mut events: Vec<Event> = Vec::new();

    let providers = create_providers(config, config_path);

    let is_reporting = false;

    let mut count = 0;
    for provider in providers {
        provider.get_events(&filter, &mut events);

        let new_count = events.len();

        if is_reporting {
            println!(
                "Got {} events from provider '{}'",
                new_count - count,
                provider.name()
            );
        }
        count = new_count;
    }

    let test_fake_category = Category::new("test", "fake");
    let filter_fakes = true;

    events = events
        .into_iter()
        .filter(|e| if filter_fakes { e.category() != test_fake_category } else { true })
        .collect();

    /*
    let mut singular_events: Vec<&Event> = Vec::new();
    let mut annual_events: Vec<&Event> = Vec::new();

    for event in &events {
        match event.kind() {
            EventKind::Singular(_) => singular_events.push(event),
            EventKind::Annual(_) | EventKind::RuleBased(_) =>
                annual_events.push(event)
        }
    }
     */

    // Separate the events using the partition adapter for the iterator:
    let (mut singular_events, annual_events): (Vec<&Event>, Vec<&Event>)
        = events.iter().partition(|event| match event.kind() {
            EventKind::Singular(_) => true,
            _ => false
        });

    if !singular_events.is_empty() {
        singular_events.sort_by(|a, b| a.year().cmp(&b.year()));
        singular_events.reverse();
        println!("On this day in history ({} events):", singular_events.len());
        for event in singular_events {
            println!("{}", event);
        }
    }

    if !annual_events.is_empty() {
        println!("\nObserved today ({} events):", annual_events.len());
        for event in annual_events {
            println!("{} ({})", event.description(), event.category());
        }
    }

    Ok(())
}

pub fn add_event(config: &Config, config_path: &Path, provider_name: &str, event: &Event) {
    let providers = create_providers(config, config_path);

    // Find provider by name
    let mut provider: Option<&dyn EventProvider> = None;
    for p in &providers {
        if p.name() == provider_name {
            provider = Some(p.as_ref());
            break;
        }
    }

    match provider {
        Some(p) => {
            if p.is_add_supported() {
                let _ = p.add_event(event);
            } else {
                eprintln!("Adding events is not supported for provider '{}'", p.name());
            }
        },
        None => {
            eprintln!("Unknown event provider '{}'", provider_name);
        }
    }
}