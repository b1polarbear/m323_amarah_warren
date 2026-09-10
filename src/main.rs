mod functionalprog;
use functionalprog::beginner::task1a::{celsius_to_fahrenheit, greet_user};

fn main() {
    println!("{}", celsius_to_fahrenheit(0.0));
    println!("{}", celsius_to_fahrenheit(100.0));

    println!("{}", greet_user("Alice"));
}
