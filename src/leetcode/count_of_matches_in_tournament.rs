struct Solution;


impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
         n-1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1(){
        let n = 7;
        let expected = 6;
        let result = Solution::number_of_matches(n);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2(){
        let n = 14;
        let expected = 13;
        let result = Solution::number_of_matches(n);
        assert_eq!(result, expected);
    }

}