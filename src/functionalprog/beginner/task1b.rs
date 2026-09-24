// Competence 1B-Beginner
// Task: Immutable data. Show how a global is manipulated by a function safely — AtomicI32, LazyLock<Mutex<T>>, const.

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{LazyLock, Mutex};


// Immutable Data - value cannot be changed after creation
const MAX_SCORE: i32 = 100;
// MAX_SCORE = 200; --> not possible

// safe global manipulation
static COUNTER: AtomicI32 = AtomicI32::new(0);
fn increased_counter() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
}

//
static DATA: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

fn change_data() {
    let mut data = DATA.lock().unwrap();
    *data += 1;
}