// Competence 2A Beginner
// Task: Ownership and borrowing, with examples.
// Show a move, an immutable borrow, a mutable borrow, and one compile error you deliberately provoked.

// MOVE (birthday gift): takes ownership of the String.
pub fn wrap_gift(gift: String) -> String {
    format!("Wrapped gift: {}", gift)
}

// IMMUTABLE BORROW (ice skates): reads with &str, the caller keeps the text.
pub fn skate_summary(model: &str, hours: u32) -> String {
    format!("{} for {} hour(s)", model, hours)
}

// MUTABLE BORROW (bike): &mut lets us change the bike.
pub struct Bike {
    pub seat_height: u32,
}

// NOT PURE: changes the bike we were given.
pub fn adjust_seat(bike: &mut Bike, new_height: u32) {
    bike.seat_height = new_height;
}

pub fn run() {
    println!("\nCompetence 2a Beginner");
    println!("----------------------");

    println!("\nMove - Birthday gift (ownership is handed over)");
    println!("-----------------------------------------------");

    let gift = String::from("book");
    println!("{}", wrap_gift(gift));
    // println!("{}", gift); // would NOT compile: gift was moved

    println!("---------------------------------------------");

    println!("\nImmutable borrow - Ice skates (read, then give back)");
    println!("-----------------------------------------------------");

    let model = String::from("Size 42 skates");
    println!("{}", skate_summary(&model, 2));
    println!("Still ours after borrowing: {}", model);

    println!("-----------------------------------------------------");

    println!("\nMutable borrow - Bike (you may change it)");
    println!("-----------------------------------------");

    let mut bike = Bike { seat_height: 60 };
    println!("Seat height before: {}", bike.seat_height);
    adjust_seat(&mut bike, 75);
    println!("Seat height after: {}", bike.seat_height);

    println!("-----------------------------------------");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_gift_returns_new_text() {
        assert_eq!(wrap_gift(String::from("book")), "Wrapped gift: book");
    }

    #[test]
    fn skate_summary_keeps_model() {
        let model = String::from("Size 42");
        assert_eq!(skate_summary(&model, 2), "Size 42 for 2 hour(s)");
        assert_eq!(model, "Size 42"); // still ours after the borrow
    }

    #[test]
    fn adjust_seat_changes_bike() {
        let mut bike = Bike { seat_height: 60 };
        adjust_seat(&mut bike, 75);
        assert_eq!(bike.seat_height, 75);
    }
}