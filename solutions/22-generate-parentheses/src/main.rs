mod tests;
fn main() {}
pub struct Solution;

// @leet start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        Self::dfs(n, 0, 0, String::new(), &mut result);

        result
    }

    pub fn dfs(
        pairs: i32,
        left_count: i32,
        right_count: i32,
        current: String,
        result: &mut Vec<String>,
    ) {
        if left_count > pairs || right_count > pairs || left_count < right_count {
            return;
        }

        if left_count == pairs && right_count == pairs {
            result.push(current);
            return;
        }

        Self::dfs(
            pairs,
            left_count + 1,
            right_count,
            format!("{current}("),
            result,
        );

        Self::dfs(
            pairs,
            left_count,
            right_count + 1,
            format!("{current})"),
            result,
        );
    }
}
// @leet end
