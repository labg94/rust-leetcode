struct Solution;

impl Solution {

    ///You have an array of floating point numbers averages which is initially empty. You are given an array nums of n integers where n is even.
    ///
    /// You repeat the following procedure n / 2 times:
    ///
    /// Remove the smallest element, minElement, and the largest element maxElement, from nums.
    /// Add (minElement + maxElement) / 2 to averages.
    /// Return the minimum element in averages.
    ///
    /// *Constraints*:
    /// - 2 <= n == nums.length <= 50
    /// - n is even.
    /// - 1 <= nums[i] <= 50
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
      let mut min_avg = f64::MAX;

        nums.sort_unstable();

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let avg = (nums[left] + nums[right]) as f64 / 2.0;

            min_avg = min_avg.min(avg);
            left+=1;
            right-=1;
        }

        min_avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case_1() {
      let actual = Solution::minimum_average(vec![7,8,3,4,15,13,4,1]);
      assert_eq!(actual, 5.5);
    }

    #[test]
    pub fn test_case_2() {
      let actual = Solution::minimum_average(vec![1,9,8,3,10,5]);
      assert_eq!(actual, 5.5);
    }

    #[test]
    pub fn test_case_3() {
      let actual = Solution::minimum_average(vec![1,2,3,7,8,9]);
      assert_eq!(actual, 5.0);
    }
}