struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        Solution::subset_xor_sum_helper(&nums, 0, 0)
    }
    
    fn subset_xor_sum_helper(nums: &Vec<i32>, index: usize, sum: i32) -> i32 {
        
        if index == nums.len() {
            return sum;
        }

        let include = Solution::subset_xor_sum_helper(nums, index + 1, sum ^ nums[index]);
        let exclude = Solution::subset_xor_sum_helper(nums, index + 1, sum);
        
        include + exclude
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let vec1 = Solution::subset_xor_sum(vec![1,3]);

        assert_eq!(vec1, 6);
    }

    #[test]
    fn case_2() {
        let vec1 = Solution::subset_xor_sum(vec![5,1,6]);

        assert_eq!(vec1, 28);
    }
    
    #[test]
    fn case_3() {
        let vec1 = Solution::subset_xor_sum(vec![3,4,5,6,7,8]);

        assert_eq!(vec1, 480);
    }
    
}
