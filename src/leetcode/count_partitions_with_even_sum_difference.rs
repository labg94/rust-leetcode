struct Solution;

impl Solution {

    ///You are given an integer array nums of length n.
    ///
    /// A partition is defined as an index i where 0 <= i < n - 1, splitting the array into two non-empty subarrays such that:
    ///
    /// Left subarray contains indices [0, i].
    /// Right subarray contains indices [i + 1, n - 1].
    /// Return the number of partitions where the difference between the sum of the left and right subarrays is even.
    ///
    ///  *Constraints*:
    ///
    /// - 2 <= n == nums.length <= 100
    /// - 1 <= nums[i] <= 100
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        for i in 1..nums.len() {
            let x = nums.split_at(i);

            let left_sum: i32 = x.0.iter().sum();
            let right_sum: i32 = x.1.iter().sum();

            if (left_sum - right_sum) % 2 == 0 {
                counter += 1;
            }

        }
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case_1() {
        let actual = Solution::count_partitions(vec![10,10,3,7,6]);
        assert_eq!(actual, 4);
    }

    #[test]
    pub fn test_case_2() {
        let actual = Solution::count_partitions(vec![1,2,2]);
        assert_eq!(actual, 0);
    }

    #[test]
    pub fn test_case_3() {
        let actual = Solution::count_partitions(vec![2,4,6,8]);
        assert_eq!(actual, 3);
    }

}