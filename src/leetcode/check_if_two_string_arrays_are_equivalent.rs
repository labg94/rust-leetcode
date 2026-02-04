struct Solution;

impl Solution {
    ///Given two string arrays word1 and word2, return `true` if the two arrays represent the same string, and `false` otherwise.
    ///
    /// A string is represented by an array if the array elements concatenated in order forms the string.
    /// **Constraints**:
    ///
    /// - 1 <= word1.length, word2.length <= 103
    ///
    /// - 1 <= word1\[i].length, word2\[i].length <= 103
    ///
    /// - 1 <= sum(word1\[i].length), sum(word2\[i].length) <= 103
    ///
    /// word1\[i] and word2\[i] consist of lowercase letters.
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
       word1.join("") == word2.join("")
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_1() {
        assert_eq!(Solution::array_strings_are_equal(vec!["ab".to_string(), "c".to_string()], vec!["a".to_string(), "bc".to_string()]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::array_strings_are_equal(vec!["a".to_string(), "cb".to_string()], vec!["ab".to_string(), "c".to_string()]), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::array_strings_are_equal(vec!["abc".to_string(), "d".to_string(), "defg".to_string()], vec!["abcddefg".to_string()]), true);
    }
}