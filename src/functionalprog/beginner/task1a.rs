// Competence 1A-Beginner
// Task: Meaning of functions and their input/output. Examples of a pure and impure function. Explain referential transparency and why a function is trivially testable.

// Pure Function - Same input, same output
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

// Impure function - accesses outside world
pub fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}

// Note: Impure functions can be impure by having side effects or accessing the outside world

/*
referential transparency
let temperature = celsius_to_fahrenheit(0.0);
let temperature = 32.0;
*/

// Test Pure function (easily testable with no external help)
#[test]
fn test_celsius_to_fahrenheit() {
    assert_eq!(celsius_to_fahrenheit(0.0), 32.0); // 0° = 32F
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
}
