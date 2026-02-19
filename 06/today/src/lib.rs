use std::error::Error;

mod birthday;

use birthday::handle_birthday;

pub fn run() -> Result<(), Box<dyn Error>> {
    handle_birthday();
    
        /*
    let mut events: Vec<Event> = Vec::new();
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
        String::from("Rust 1.92.0 released"),
        Category::new("programming", "rust"),
    ));
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
        String::from("Rust 1.0.0 released"),
        Category::new("programming", "rust"),
    ));
    for event in events {
        println!("{}: {}", event.year(), event.description);
    }
    */

    Ok(())
}
