struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result = vec![' '; indices.len()];

        for (ch, &target_index) in s.chars().zip(indices.iter()) {
            result[target_index as usize] = ch;
        }

        result.into_iter().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case_1() {
        assert_eq!(Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]), "leetcode".to_string());
    }

    #[test]
    pub fn test_case_2() {
        assert_eq!(Solution::restore_string("abc".to_string(), vec![0, 1, 2]), "abc".to_string());
    }
}