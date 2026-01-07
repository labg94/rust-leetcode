use crate::leetcode_solution;

leetcode_solution! {
    fn reverse_prefix(s: String, k: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars[..k as usize].reverse();
        chars.into_iter().collect()
    }
    
    tests {
        test_case_1: ("abcd".to_string(), 2) => "bacd".to_string(),
        test_case_2: ("xyz".to_string(), 3) => "zyx".to_string(),
        test_case_3: ("hey".to_string(), 1) => "hey".to_string(),
    }
}
