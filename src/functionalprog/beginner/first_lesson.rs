pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_three_is_five(){
        assert_eq!(add(2, 3), 5);
    }
}