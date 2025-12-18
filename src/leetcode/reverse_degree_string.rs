struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.chars()
            .enumerate()
            .map(|(i,c)| (i as i32+1) *  (26 - (c as i32 - 'a' as i32)))
            .sum()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let response = Solution::reverse_degree("abc".to_string());
        
        assert_eq!(response, 148);
    }

    #[test]
    fn case_2() {
        let response = Solution::reverse_degree("zaza".to_string());

        assert_eq!(response, 160);
    }


}
