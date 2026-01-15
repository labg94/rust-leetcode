struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut depth = 0;

        for byte in s.bytes() {
            match byte {
                b'(' => {
                    if depth > 0 {
                        result.push('(');
                    }
                    depth += 1;
                }
                b')' => {
                    depth -= 1;
                    if depth > 0 {
                        result.push(')');
                    }
                }
                _ => {}
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let response = Solution::remove_outer_parentheses("(()())(())".to_string());

        assert_eq!(response, "()()()");
    }
    #[test]
    fn case_2() {
        let response = Solution::remove_outer_parentheses("(()())(())(()(()))".to_string());

        assert_eq!(response, "()()()()(())");
    }
    #[test]
    fn case_3() {
        let response = Solution::remove_outer_parentheses("()()".to_string());

        assert_eq!(response, "");
    }
}
