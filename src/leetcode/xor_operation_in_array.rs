struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut acum= 0;
        for i in 0..n {
            acum ^= start + 2*i;
        }
        acum
    }



}


#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test_xor_operation() {
        assert_eq!(Solution::xor_operation(5, 0), 8);
    }
    
    #[test]
    fn test_xor_operation2() {
        assert_eq!(Solution::xor_operation(4, 3), 8);
    }   

}