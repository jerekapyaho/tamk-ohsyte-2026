use chrono::{Datelike, NaiveDate};

#[derive(Debug)]
pub enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug)]
pub struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}

impl Event {
    fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Singular(date),
            description,
            category,
        }
    }

    pub fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }
    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => MonthDay {
                month: date.month(),
                day: date.day(),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MonthDay {
    month: u32,
    day: u32,
}

impl MonthDay {
    fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }
    fn from_str(s: &str) -> Self {
        assert!(s.len() == 4);
        let month_string = &s[..2];
        let month = month_string.parse().unwrap();
        let day: u32 = s[2..].parse().unwrap();
        MonthDay { month, day }
    }
}

#[derive(Debug)]
pub struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }
    fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }
}

fn main() {
    if let Err(e) = today::run() {
        eprintln!("Error: {}", e);
        return;
    }

}
