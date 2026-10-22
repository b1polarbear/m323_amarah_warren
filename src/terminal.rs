use std::io::{self, Write};

pub fn run_menu() {
    println!("Welcome to Module 323!");
    println!("What would you like to learn about today?");
    println!("1. Functional Programming");
    println!("2. Functions");
    println!("3. Other topic");
    println!("4. Other topic");

    print!("Choose a topic: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => functional_programming_menu(),
        // "2" => functions_menu(),
        // "3" => println!("Topic 3"),
        // "4" => println!("Topic 4"),
        _ => println!("Invalid choice"),
    }
}

fn functional_programming_menu() {
    println!("\nFunctional Programming");
    println!("1. Beginner");
    println!("2. Intermediate");
    println!("3. Advanced");
    println!("4. Expert");

    print!("Choose difficulty: ");
    io::stdout().flush().unwrap();

    let mut difficulty = String::new();
    io::stdin().read_line(&mut difficulty).unwrap();

    match difficulty.trim() {
        "1" => functional_beginner_menu(),
        // "2" => functional_intermediate_menu(),
        // "3" => functional_advanced_menu(),
        // "4" => functional_expert_menu(),
        _ => println!("Invalid difficulty"),
    }
}

fn functional_beginner_menu() {
    println!("\nFunctional Programming - Beginner");
    println!("1A. Task 1A");
    // println!("1B. Task 1B");

    print!("Choose a task: ");
    io::stdout().flush().unwrap();

    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();

    match task.trim().to_lowercase().as_str() {
        "1a" => crate::functionalprog::beginner::task1a::run(),
        // "1b" => crate::functionalprog::beginner::task1b::run(),
        _ => println!("Invalid task"),
    }
}
