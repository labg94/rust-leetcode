struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut counter = 0;
        let mut l_counter = 0;
        let mut r_counter = 0;

        for x in s.chars() {
            if x == 'L' {
                l_counter += 1;
            }else if x == 'R' {
                r_counter += 1;
            }
            
            if l_counter == r_counter {
                counter += 1;
            }
        }
        
        
        counter
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case_1() {
        assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
    }
        
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_string()), 2);
    }
           
    #[test]
    fn test_case_3() {
        assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
    }
    
}