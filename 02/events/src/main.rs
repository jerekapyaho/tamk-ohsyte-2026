fn main() {
    let events = [
        (1996_01_23, "JDK 1.0 released"), // the first version of Java
        (2008_12_03, "Python 3.0 released"), // the modern version of Python
        (2015_05_15, "Rust 1.0.0 released"),
    ];
    println!("{:#?}", events);
    println!("{} events", events.len());
    println!("Most important event: {:?}", events[2]);
}
