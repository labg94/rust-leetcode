struct Solution;

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let reversed = Self::reverse_integer(n);
        reversed.abs_diff(n) as i32
    }

    fn reverse_integer(mut n: i32) -> i32 {
        let mut reversed = 0;
        while n > 0 {
            reversed = reversed * 10 + n % 10;
            n /= 10;
        }
        reversed
    }
}

#[cfg(test)]
mod tests{
    
    use super::*;
    
    
    #[test]
    fn test_case_1(){
        assert_eq!(Solution::mirror_distance(25), 27);
    }
    
    #[test]
    fn test_case_2(){
        assert_eq!(Solution::mirror_distance(10), 9);
    }
    
    
}