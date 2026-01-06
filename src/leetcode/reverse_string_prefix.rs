struct Solution;

impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {

        let mut chars: Vec<char> = s.chars().collect();
        chars[..k as usize].reverse();
        chars.into_iter().collect()


    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test_case_1() {
        assert_eq!(Solution::reverse_prefix("abcd".to_string(), 2), "bacd".to_string());
    }    
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::reverse_prefix("xyz".to_string(), 3), "zyx".to_string());
    }  
    #[test]
    fn test_case_3() {
        assert_eq!(Solution::reverse_prefix("hey".to_string(), 1), "hey".to_string());
    }
}