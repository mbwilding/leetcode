#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let result = Solution::fizz_buzz(15);

        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ];
        assert_eq!(result, expected);
    }
}
