pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn add_negatives() {
        let ans = add(-1, -4);
        assert_eq!(ans, -5);
    }
}