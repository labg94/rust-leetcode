struct Solution;


impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut nums = nums;
        for _i in 0..k {
            let mut min = i32::MAX;
            let mut index = 0;
            
            for j in 0..nums.len() {
                if nums[j] < min {
                    min = nums[j];
                    index = j;
                }
            }
            nums[index] *= multiplier;
        }
        
        
        nums
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case_1() {
        assert_eq!(Solution::get_final_state(vec![2,1,3,5,6], 5, 2), vec![8,4,6,5,6])
    }
      
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::get_final_state(vec![1,2], 3, 4), vec![16,8])
    }
    
    
}

