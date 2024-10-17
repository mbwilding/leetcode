#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mut result: Vec<String> = vec![];
        Solution::dfs(1, 0, 0, String::new(), &mut result);
        assert_eq!(result, vec!["()"]);
    }

    #[test]
    fn test_2() {
        let mut result: Vec<String> = vec![];
        Solution::dfs(2, 0, 0, String::new(), &mut result);
        assert_eq!(result, vec!["(())", "()()"]);
    }

    #[test]
    fn test_3() {
        let mut result: Vec<String> = vec![];
        Solution::dfs(3, 0, 0, String::new(), &mut result);
        assert_eq!(result, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }
}
