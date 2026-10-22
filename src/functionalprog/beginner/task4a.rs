// Competence 4A-Beginner
// Task: The Rust toolchain:
// rustup, cargo, clippy, rustfmt, cargo doc, Cargo.lock
// and point at your own CI pipeline using them

pub fn run() {
    println!("\nCompetence 4A-Beginner");
    println!("----------------------");

    println!("\nThe Rust Toolchain");
    println!(
        "What do the commands do?\nCommands: rustup, cargo, clippy, rustfmt, cargo doc, Cargo.lock"
    );
    println!("---------------------------------------");
    println!("rustup: Installs and Updates Rust");
    println!("cargo: Used to build, run, run tests, manages dependencies");
    println!(
        "clippy: Linting-Tool used to review code and check for mistakes (helps write clean code)"
    );
    println!("rustfmt: Formats code (layout consistency)");
    println!("cargo doc: Builds HTML docs from /// comments");
    println!("Cargo.lock: Records the exact version of the libraries my project uses");
    println!("---------------------------------------");

    println!("\nCI Pipeline");
    println!("The CI Pipeline will be in a different file.");
}
