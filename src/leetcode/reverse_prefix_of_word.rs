struct Solution;

impl Solution {
    fn reverse_prefix(word: String, ch: char) -> String {
        let Some((byte_idx, _)) = word.char_indices().find(|&(_, c)| c == ch) else {
            return word;
        };

        let split = byte_idx + ch.len_utf8();
        let (prefix, suffix) = word.split_at(split);

        prefix.chars().rev().collect::<String>() + suffix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::reverse_prefix("abcdefd".to_string(), 'd'),
            "dcbaefd".to_string()
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::reverse_prefix("xyxzxe".to_string(), 'z'),
            "zxyxxe".to_string()
        )
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::reverse_prefix("abcd".to_string(), 'z'),
            "abcd".to_string()
        )
    }
}
