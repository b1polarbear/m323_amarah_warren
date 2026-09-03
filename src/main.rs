use std::io;

use crate::functionalprog::beginner::first_lesson::add;

mod functionalprog;

fn main() {
    loop {
        println("\nCompetency Grid Menu");
        println("--------------------");
        println("1. Beginner");
        println("2. Intermediate");
        println("3. Advanced");
        println("4. Expert");
        println("5. Quit");
        println("\nChoose (1-5): ");

        io::Write::flush(&mut std::io::stdout()).ok();

        let choice = read_choice();

        match choice {
            1 => select_row(&Level::Beginner),
            2 => select_row(&Level::Intermediate),
            3 => select_row(&Level::Advanced),
            4 => select_row(&Level::Expert),
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

enum Level {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}