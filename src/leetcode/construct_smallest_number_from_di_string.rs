struct Solution;

impl Solution {
    /// You are given a 0-indexed string pattern of length n consisting of the characters 'I' meaning increasing and 'D' meaning decreasing.
    ///
    /// A 0-indexed string num of length n + 1 is created using the following conditions:
    ///
    /// num consists of the digits '1' to '9', where each digit is used at most once.
    /// If pattern[i] == 'I', then num[i] < num[i + 1].
    /// If pattern[i] == 'D', then num[i] > num[i + 1].
    /// Return the lexicographically smallest possible string num that meets the conditions.
    /// *Constraints:*
    ///
    /// - 1 <= pattern.length <= 8
    /// - pattern consists of only the letters 'I' and 'D'.
    pub fn smallest_number(pattern: String) -> String {
        let mut response = String::new();
        let mut num_stack = Vec::with_capacity(9);
        let chars = pattern.chars().collect::<Vec<char>>();

        for i in 0..=chars.len() {
            num_stack.push((b'1'+ i as u8) as char);

            if i == chars.len() || chars[i] == 'I'{
                while !num_stack.is_empty() {
                    response.push( num_stack.pop().unwrap())
                }
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case_1() {
        assert_eq!(
            Solution::smallest_number("IIIDIDDD".to_string()),
            "123549876".to_string()
        );
    }

    #[test]
    pub fn test_case_2() {
        assert_eq!(
            Solution::smallest_number("DDD".to_string()),
            "4321".to_string()
        );
    }
}
