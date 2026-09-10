// Competence 1A-Beginner
// Task: Meaning of functions and their input/output. Examples of a pure and impure function. Explain referential transparency and why a function is trivially testable.




// Pure Function - Same input, same output
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

// Impure function - accesses outside world (using console)
pub fn greet_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Test Pure function
#[test]
fn test_celsius_to_fahrenheit() {
    assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
}