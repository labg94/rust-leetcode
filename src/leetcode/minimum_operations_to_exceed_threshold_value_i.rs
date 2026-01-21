struct Solution;

impl Solution {

    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().filter(|&&x| x <k).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
     assert_eq!(Solution::min_operations(vec![2,11,10,1,3], 10), 3)
    }

    #[test]
    fn test_case_2() {
     assert_eq!(Solution::min_operations(vec![1,1,2,4,9], 1), 0)
    }
    #[test]
    fn test_case_3() {
     assert_eq!(Solution::min_operations(vec![1,1,2,4,9], 9), 4)
    }

}