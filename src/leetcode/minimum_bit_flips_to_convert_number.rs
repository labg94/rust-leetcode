struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let xor = start ^ goal;
        xor.count_ones() as i32
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let response = Solution::min_bit_flips(10,7);

        assert_eq!(response, 3);
    }  
    
    #[test]
    fn case_2() {
        let response = Solution::min_bit_flips(3,4);

        assert_eq!(response, 3);
    }



}
