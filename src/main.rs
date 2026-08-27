use std::io;

use crate::functionalprog::beginner::first_lesson::add;

mod functionalprog;

fn main() {
    // Borrowing
    let title: &str = "Rust Programming";
    let year: i32 = 2026;

    let mut name: String = String::from("John Doe");

    //read from console (user input)
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("2 + 3 = {}", add(2, 3));
}
