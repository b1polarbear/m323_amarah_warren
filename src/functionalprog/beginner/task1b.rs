// Competence 1B-Beginner
// Task: Immutable data
// Show how a global is manipulated by a function safely
// AtomicI32, LazyLock<Mutex<T>>, const

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{LazyLock, Mutex};

// const: fixed baggage fee per kg, printed on the airline website. Never changes.
pub const BAGGAGE_FEE_PER_KG: i32 = 2;

// AtomicI32: the airline's current base price per ticket. The airline can change it,
// and each change happens in one safe step.
static BASE_PRICE: AtomicI32 = AtomicI32::new(50);

// LazyLock<Mutex<T>>: a booking log, created on first use. Mutex allows only one writer at a time.
static BOOKING_LOG: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

// PURE: the total depends only on the inputs. It reads the const, which never changes.
pub fn total_price(tickets: i32, baggage_kg: i32, base: i32) -> i32 {
    tickets * base + baggage_kg * BAGGAGE_FEE_PER_KG
}

// IMPURE: reads BASE_PRICE, so the result depends on global state.
pub fn current_total(tickets: i32, baggage_kg: i32) -> i32 {
    total_price(tickets, baggage_kg, BASE_PRICE.load(Ordering::SeqCst))
}

// IMPURE: changes BASE_PRICE.
pub fn airline_raises_price(by: i32) {
    BASE_PRICE.fetch_add(by, Ordering::SeqCst);
}

// IMPURE: changes BOOKING_LOG.
pub fn log_booking(text: &str) {
    BOOKING_LOG.lock().unwrap().push(text.to_string());
}

pub fn run() {
    println!("-----------------------");
    println!("\nCompetence 1B-Beginner");
    println!("-----------------------");

    println!("const: baggage fee per kg = {}", BAGGAGE_FEE_PER_KG);
    println!("Pure total for 2 tickets, 10 kg, base 50: {}", total_price(2, 10, 50));

    println!("\nAtomicI32: current total before raise = {}", current_total(2, 10));
    airline_raises_price(5);
    println!("AtomicI32: current total after raise by 5 = {}", current_total(2, 10));

    println!("\nLazyLock<Mutex<T>>: booking log");
    log_booking("Booking: 2 tickets, 10 kg");
    println!("Bookings logged = {}", BOOKING_LOG.lock().unwrap().len());
}

// Test (only for the pure function)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_price_returns120() {
        // 2 tickets x 50 = 100, plus 10 kg x 2 = 20, total 120
        assert_eq!(total_price(2, 10, 50), 120);
    }
}