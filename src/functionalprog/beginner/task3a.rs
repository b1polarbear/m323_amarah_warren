// Competence 3A-Beginner
// Task: Declarative vs imperative, an example of both.
// The same function twice,
// and a test asserting both versions agree.

// Imperative: step by step, with a changing variable
pub fn sum_of_evens_imperative(numbers: &[i32]) -> i32 {
    let mut total = 0; // starts at 0
    for &n in numbers {
        if n % 2 == 0 {
            // only even numbers
            total += n; // update
        }
    }
    total
}

// Declarative: describe what we want (keep evens, then add them)
pub fn sum_of_evens_declarative(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0) // keep only the even numbers
        .sum() // add them up
}

pub fn run() {
    let numbers = [1, 2, 3, 4, 5, 6];

    println!("\nCompetence 3A-Beginner");
    println!("----------------------");

    println!("\nDeclarative vs imperative");
    println!(
        "Imperative: explain step by step what needs to be done. For example: A cookbook gives you step by step instructions on how to make spaghetti."
    );
    println!(
        "Declarative: describe what we want, and let the pc figure it out. For example: You get in to a Taxi and tell him 'Take me to Migros'. You don't care what route he takes, as long as you get to your destination."
    );

    println!("\nExample with the list: 1,2,3,4,5,6");
    println!("Imperative: {}", sum_of_evens_imperative(&numbers));
    println!("Declarative: {}", sum_of_evens_declarative(&numbers));

    println!("\nThey both return the same value.");
    println!("-----------------------------------")
}

#[cfg(test)]
mod tests {
    use super::*; // bring both functions into this test module

    #[test]
    fn both_versions_agree() {
        let numbers = [1, 2, 3, 4, 5, 6];
        assert_eq!(
            sum_of_evens_imperative(&numbers),
            sum_of_evens_declarative(&numbers)
        );
        // the answer itself must be correct: 2 + 4 + 6 = 12
        assert_eq!(sum_of_evens_declarative(&numbers), 12);
    }

    #[test]
    fn empty_list_gives_zero() {
        let numbers: [i32; 0] = [];
        assert_eq!(sum_of_evens_imperative(&numbers), 0);
        assert_eq!(sum_of_evens_declarative(&numbers), 0);
    }
}
