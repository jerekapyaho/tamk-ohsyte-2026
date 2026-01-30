use std::env;

use cgisf_lib::{SentenceConfigBuilder, gen_sentence};
use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn from_i32(number: i32) -> Self {
        match number {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("Invalid month number: {}", number),
        }
    }
}

struct Date {
    year: i16,
    month: Month,
    day: u8,
}

impl Date {
    fn new(year: i16, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }

    fn random() -> Self {
        let mut rng = rand::rng();
        Self {
            year: rng.random_range(1900..2099),
            month: Month::from_i32(rng.random_range(1..=12)),
            day: rng.random_range(1..=31),
        }
    }

    fn is_valid(&self) -> bool {
        self.day >= 1 && self.day <= Self::day_count(self.month, self.year)
    }

    fn day_count(month: Month, year: i16) -> u8 {
        match month {
            Month::April | Month::June | Month::September | Month::November => 30,
            Month::February => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 31,
        }
    }

    fn is_leap_year(year: i16) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: Month,
    day: u8,
}

impl MonthDay {
    fn from_str(s: &str) -> Self {
        assert!(s.len() == 4);
        let month_string = &s[..2];
        let month = Month::from_i32(month_string.parse().unwrap());
        let day: u8 = s[2..].parse().unwrap();
        MonthDay { month, day }
    }
}

#[derive(Debug)]
struct Category {
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

struct Event {
    date: Date,
    description: String,
    category: Category,
}

impl Event {
    fn new(date: Date, description: String, category: Category) -> Self {
        Event {
            date,
            description,
            category,
        }
    }

    fn month_day(&self) -> MonthDay {
        MonthDay {
            month: self.date.month,
            day: self.date.day,
        }
    }

    fn random() -> Option<Self> {
        let description = gen_sentence(SentenceConfigBuilder::random().build());
        let date = Date::random();
        let category = Category::new("test", "random");
        if date.is_valid() {
            Some(Self {
                date,
                description,
                category,
            })
        } else {
            None
        }
    }
}

fn main() {
    let mut events = vec![
        Event::new(
            Date::new(2026, Month::January, 29),
            String::from("04 completed"),
            Category::new("programming", "ohsyte"),
        ),
        Event::new(
            Date::new(1996, Month::January, 23),
            String::from("JDK 1.0 released"),
            Category::new("programming", "java"),
        ),
        Event::new(
            Date::new(2008, Month::December, 3),
            String::from("Python 3.0 released"),
            Category::new("programming", "python"),
        ),
        Event::new(
            Date::new(2015, Month::May, 15),
            String::from("Rust 1.0.0 released"),
            Category::new("programming", "rust"),
        ),
        Event::new(
            Date::new(2025, Month::September, 16),
            String::from("Java 25 released"),
            Category::new("programming", "java"),
        ),
        Event::new(
            Date::new(2025, Month::October, 7),
            String::from("Python 3.14 released"),
            Category::new("programming", "python"),
        ),
        Event::new(
            Date::new(2025, Month::December, 11),
            String::from("Rust 1.92.0 released"),
            Category::new("programming", "rust"),
        ),
    ];

    // Add some random events
    let mut valid_count = 0;
    const MAX_COUNT: usize = 10_000;
    while valid_count < MAX_COUNT {
        if let Some(candidate) = Event::random() {
            events.push(candidate);
            valid_count += 1;
        }
    }

    let args: Vec<String> = env::args().collect();
    let month_day = MonthDay::from_str(&args[1]);
    //println!("{:#?}", month_day);

    let mut any_luck = false; // Boolean flag
    for event in events {
        if event.month_day() == month_day {
            println!(
                "{}: {} ({:#?})",
                event.date.year, event.description, event.category
            );
            any_luck = true;
        }
    }

    if !any_luck {
        println!("No events for {:#?}", month_day);
    }
}
