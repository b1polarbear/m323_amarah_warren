// Competence 3A-Beginner
// Task: Declarative vs imperative, an example of both. 
// The same function twice, 
// and a test asserting both versions agree.

// Imperative: step by step, with a changing variable
pub fn sum_of_evens_imperative(numbers: &[i32]) -> i32 {
    let mut total = 0; // starts at 0
    for &n in numbers {
        if n % 2 == 0 { // only even numbers
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
        .sum() // add them up (the return type i32 tells Rust what to sum into)
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
