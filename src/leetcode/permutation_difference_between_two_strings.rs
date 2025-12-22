struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut positions = [0i32; 26];

        for (i, ch) in s.chars().enumerate() {
            positions[(ch as u8 - b'a') as usize] = i as i32;
        }

        t.chars()
            .enumerate()
            .map(|(i, ch)| {
                let s_pos = positions[(ch as u8 - b'a') as usize];
                (s_pos - i as i32).abs()
            })
            .sum()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let response = Solution::find_permutation_difference("abc".to_string(), "bac".to_string());

        assert_eq!(response, 2);
    }

    #[test]
    fn case_2() {
        let response =
            Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string());

        assert_eq!(response, 12);
    }
}
