// Competence 1B-Beginner
// Task: Immutable data.
// Show how a global is manipulated by a function safely:
// AtomicI32, LazyLock<Mutex<T>>, const.

use std::sync::{
    LazyLock, Mutex,
    atomic::{AtomicI32, Ordering},
};

// 1. AtomicI32
// A global integer that can be changed safely.
static COUNTER: AtomicI32 = AtomicI32::new(0);

fn increase_counter() {
    COUNTER.fetch_add(1, Ordering::SeqCst); // special operation
}

// 2. LazyLock<Mutex<T>> A global String
// LazyLock = creates the value when it is first needed.
// Mutex = makes changing the String safe.
static MESSAGE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::from("Hello")));
fn change_message() {
    // Lock the Mutex so we can safely modify the String.
    let mut message = MESSAGE.lock().unwrap();
    message.push_str(" Rust!");
}

// 3. const (constant cannot be changed)
const MAX_SCORE: i32 = 100;
// NOT POSSIBLE: MAX_SCORE = 200;

// function for main
pub fn run_task1b() {
    // AtomicI32
    println!("Counter before: {}", COUNTER.load(Ordering::SeqCst));

    increase_counter();
    increase_counter();

    println!("Counter after: {}", COUNTER.load(Ordering::SeqCst));

    // LazyLock<Mutex<String>>
    change_message();

    let message = MESSAGE.lock().unwrap();
    println!("Message: {}", *message);

    // const
    println!("Maximum score: {}", MAX_SCORE);
}
