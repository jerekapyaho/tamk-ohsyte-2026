use rand::Rng;
use rand::RngExt;

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

pub struct Date {
    year: i16,
    month: Month,
    day: u8,
}

impl Date {
    pub fn new(year: i16, month: Month, day: u8) -> Option<Self> {
        let valid_day = 1..=Self::day_count(month, year);
        if !valid_day.contains(&day) {
            return None;
        }
        Some(Self { year, month, day })
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

impl Date {
    pub fn random() -> Option<Date> {
        let mut rng = rand::rng();
        Date::new(
            rng.random_range(1900..2099),
            Month::from_i32(rng.random_range(1..=12)),
            rng.random_range(1..=31),
        )
    }
}

fn main() {
    let mut total_count = 0;
    let mut valid_count = 0;
    const MAX_COUNT: usize = 100_000;
    let mut random_dates = Vec::with_capacity(MAX_COUNT);
    while valid_count < MAX_COUNT {
        total_count += 1;
        if let Some(candidate) = Date::random() {
            random_dates.push(candidate);
            valid_count += 1;
        }
    }
    println!(
        "Generated {} random date candidates to get {} valid dates",
        total_count,
        random_dates.len()
    );
}
