struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        sum % k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1(){
        assert_eq!(Solution::min_operations(vec![3,9,7], 5), 4)
    }

    #[test]
    fn test_case_2(){
        assert_eq!(Solution::min_operations(vec![4,1,3], 4), 0)
    }

    #[test]
    fn test_case_3(){
        assert_eq!(Solution::min_operations(vec![3,2], 6), 5)
    }

}