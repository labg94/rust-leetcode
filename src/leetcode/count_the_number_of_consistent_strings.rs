struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut mask = 0;
        
        allowed.chars().for_each(|c| mask |= 1 << (c as u32 - 'a' as u32));
        
        words.into_iter().filter(|w| w.chars().all(|c| mask & (1 << (c as u32 - 'a' as u32)) != 0)).count() as i32
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let response = Solution::count_consistent_strings("ab".to_string(), vec!["ad", "bd", "aaab", "baa", "badab"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>() );

        assert_eq!(response, 2);
    }
    
    #[test]
    fn case_2() {
        let response = Solution::count_consistent_strings("abc".to_string(), vec!["a","b","c","ab","ac","bc","abc"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>() );

        assert_eq!(response, 7);
    }
    
    #[test]
    fn case_3() {
        let response = Solution::count_consistent_strings("cad".to_string(), vec!["cc","acd","b","ba","bac","bad","ac","d"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>() );

        assert_eq!(response, 4);
    }

    
    
}
