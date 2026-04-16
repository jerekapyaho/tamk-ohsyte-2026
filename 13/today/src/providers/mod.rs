use crate::events::{Category, Event};
use chrono::{Local, NaiveDate};

use crate::filters::EventFilter;

pub mod csvfile;
pub mod sqlite;
pub mod textfile;
pub mod web;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>);
    fn is_add_supported(&self) -> bool { false }
    fn add_event(&self, event: &Event) -> Result<(), EventProviderError>;
    fn kind(&self) -> String;
}

pub enum EventProviderError {
    OperationNotSupported,
    OperationFailed,
}

pub struct SimpleProvider {
    name: String,
}

impl SimpleProvider {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl EventProvider for SimpleProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();

        let test_event_1 = Event::new_singular(
            today,
            String::from("Test event 1 for today"),
            Category::from_primary("test"),
        );
        events.push(test_event_1);

        let test_event_2 = Event::new_singular(
            today,
            String::from("Test event 2 for today"),
            Category::from_primary("test"),
        );
        events.push(test_event_2);
    }

    fn add_event(&self, _event: &Event) -> Result<(), EventProviderError> {
        Err(EventProviderError::OperationNotSupported)
    }

    fn kind(&self) -> String { String::from("simple") }
}
