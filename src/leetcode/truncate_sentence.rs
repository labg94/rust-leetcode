struct Solution;

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.split_whitespace().take(k as usize).collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::truncate_sentence("Hello how are you Contestant".to_string(), 4), "Hello how are you".to_string());
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::truncate_sentence("What is the solution to this problem".to_string(), 4), "What is the solution".to_string());
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::truncate_sentence("chopper is not a tanuki".to_string(), 5), "chopper is not a tanuki".to_string());
    }
}