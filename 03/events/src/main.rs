struct Date {
    year: i16,
    month: u8,
    day: u8,
}

struct Event {
    date: Date,
    description: String,
}

fn main() {
    let events = [
        Event {
            date: Date {
                year: 1996,
                month: 1,
                day: 23,
            },
            description: String::from("JDK 1.0 released"),
        },
        Event {
            date: Date {
                year: 2008,
                month: 12,
                day: 3,
            },
            description: String::from("Python 3.0 released"),
        },
        Event {
            date: Date {
                year: 2015,
                month: 5,
                day: 15,
            },
            description: String::from("Rust 1.0.0 released"),
        },
        Event {
            date: Date {
                year: 2025,
                month: 9,
                day: 16,
            },
            description: String::from("Java 25 released"),
        },
        Event {
            date: Date {
                year: 2025,
                month: 10,
                day: 7,
            },
            description: String::from("Python 3.14 released"),
        },
        Event {
            date: Date {
                year: 2025,
                month: 12,
                day: 11,
            },
            description: String::from("Rust 1.92.0 released"),
        },
    ];

    // Get the month and day from our target date:
    let date = Date { year: 2015, month: 5, day: 15 };

    let month_day = (date.month, date.day);

    let mut any_luck = false;

    for event in events {
        let event_month_day = (event.date.month, event.date.day);
        if event_month_day == month_day {
            println!("{}: {}", event.date.year, event.description);
            any_luck = true;
        }
    }

    if !any_luck {
        println!("No events found.");
    }
}
