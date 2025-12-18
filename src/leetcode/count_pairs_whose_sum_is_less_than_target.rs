struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut left = 0;
        let mut right = sorted_nums.len() - 1;
        let mut counter = 0;

        while left < right {
            if sorted_nums[left] + sorted_nums[right] < target {
                counter += (right - left) as i32;
                left += 1;
            } else {
                right -= 1;
            }
        }

        counter

    }

    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let response = Solution::count_pairs(vec![-1,1,2,3,1], 2 );

        assert_eq!(response, 3);
    }
    
    #[test]
    fn case_2() {
        let response = Solution::count_pairs(vec![-6,2,5,-2,-7,-1,3], -2 );

        assert_eq!(response, 10);
    }
    


}
