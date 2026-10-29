// Competence 1B-Beginner
// Task: Immutable data
// Show how a global is manipulated by a function safely
// AtomicI32, LazyLock<Mutex<T>>, const

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{LazyLock, Mutex};

// AtomicI32: global number that can be changed safely
static COUNTER: AtomicI32 = AtomicI32::new(0);

// LazyLock<Mutex<T>>: global list created on first use.
// Mutex makes sure only one caller edits the list at once.
static LOG: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

// const: fixed value that never changes
pub const MAX_VALUE: i32 = 100;

// PURE: same input, same output, no global state
pub fn clamp_to_max(value: i32) -> i32 {
    value.min(MAX_VALUE)
}

// IMPURE: changes COUNTER
pub fn increment_counter() -> i32 {
    COUNTER.fetch_add(1, Ordering::SeqCst) + 1
}

// IMPURE: reads COUNTER
pub fn current_counter() -> i32 {
    COUNTER.load(Ordering::SeqCst)
}

// IMPURE: changes LOG
pub fn log_message(message: &str) {
    LOG.lock().unwrap().push(message.to_string());
}

pub fn run() {
    println!("----------------------");
    println!("\nCompetence 1B-Beginner");
    println!("----------------------");

    println!("\nImmutable Data:");
    println!("Immutable means 'cannot be changed'.");
    println!("Eg. History.");

    println!("\nMutable Data:");
    println!("Mutable is the opposite, so it can be changed.");
    println!("Eg. Whiteboard.");

    println!("\nIn Rust:");

    println!("const: A constant can never be changed.");
    println!("clamp_to_max(500) = {}", clamp_to_max(500));

    println!("\nAtomicI32: This is a number that can be changed safely.");
    println!("increment_counter() = {}", increment_counter());
    println!("current_counter() = {}", current_counter());

    println!("\nLazyLock<Mutex<T>>: This is a list or value that can also be changed.");
    log_message("hello");
    println!("-----------------------------------------------------------------------");
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_works() {
        assert_eq!(clamp_to_max(5), 5);
        assert_eq!(clamp_to_max(500), 100);
    }
}
