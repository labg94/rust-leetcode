use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let total = 3 * (1 << n - 1);

        if k > total {
            return "".to_string();
        }

        let mut result = vec!['a'; n as usize];

        let mut next_smallest = HashMap::with_capacity(3usize);
        next_smallest.insert('a', 'b');
        next_smallest.insert('b', 'a');
        next_smallest.insert('c', 'a');

        let mut next_greates = HashMap::with_capacity(3usize);
        next_greates.insert('a', 'c');
        next_greates.insert('b', 'c');
        next_greates.insert('c', 'b');

        let start_a = 1;
        let start_b = start_a + (1 << n - 1);
        let start_c = start_b + (1 << n - 1);

        if k < start_b {
            result[0] = 'a';
            k -= start_a;
        } else if k < start_c {
            result[0] = 'b';
            k -= start_b;
        } else {
            result[0] = 'c';
            k -= start_c;
        }

        for char_index in 1..n {
            let mid_point = 1 << (n - char_index - 1);

            if k < mid_point {
                result[char_index as usize] = next_smallest[&result[char_index as usize - 1]];
            } else {
                result[char_index as usize] = next_greates[&result[char_index as usize - 1]];
                k -= mid_point;
            }
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case_1() {
        let actual = Solution::get_happy_string(1, 3);
        assert_eq!(actual, "c");
    }

    #[test]
    pub fn test_case_2() {
        let actual = Solution::get_happy_string(1, 4);
        assert_eq!(actual, "");
    }

    #[test]
    pub fn test_case_3() {
        let actual = Solution::get_happy_string(3, 9);
        assert_eq!(actual, "cab");
    }
}
