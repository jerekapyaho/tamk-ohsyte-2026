use std::fs::File;
fn main() {
    let file_result = File::open("events.txt");
    let file = match file_result {
        Ok(f) => f,
        Err(e) => {
            panic!("Error opening file: {:?}", e);
        }
    };
    // If we make it here, we can start reading from `file`
    println!("Jee!");
}
