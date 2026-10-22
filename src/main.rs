// use std::io;
mod functionalprog;
mod terminal;

// use crate::functionalprog::beginner::first_lesson::add;
// use functionalprog::beginner::task1a::{celsius_to_fahrenheit, greet_user};

fn main() {
    terminal::run_menu();

    /*
    // Borrowing
    let title: &str = "Rust Programming";
    let year: i32 = 2026;

    let mut name: String = String::from("John Doe");

    //read from console (user input)
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("2 + 3 = {}", add(2, 3));
    println!("{} {}", title, year);
     */
}
