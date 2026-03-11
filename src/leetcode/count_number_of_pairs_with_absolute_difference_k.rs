struct Solution;

impl Solution {

    ///Given an integer array nums and an integer k, return the number of pairs (i, j) where i < j such that |nums[i] - nums[j]| == k.
    ///
    /// The value of |x| is defined as:
    ///
    /// x if x >= 0.
    /// -x if x < 0.
    /// *Constraints:*
    //
    // - 1 <= nums.length <= 200
    // - 1 <= nums[i] <= 100
    // - 1 <= k <= 99
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = [0; 101];
        let mut count = 0;

        for num in nums {
            if num - k >= 1 {
                count += freq[(num - k) as usize];
            }
            if num + k <= 100 {
                count += freq[(num + k) as usize];
            }

            freq[num as usize] += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test_case_1() {
          let actual = Solution::count_k_difference(vec![1,2,2,1], 1);
          assert_eq!(actual, 4);
    }
    #[test]
    pub fn test_case_2() {
          let actual = Solution::count_k_difference(vec![1,3], 3);
          assert_eq!(actual, 0);
    }

    #[test]
    pub fn test_case_3() {
          let actual = Solution::count_k_difference(vec![3,2,1,5,4], 2);
          assert_eq!(actual, 3);
    }
}