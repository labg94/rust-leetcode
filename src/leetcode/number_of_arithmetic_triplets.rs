struct Solution;

impl Solution {

    ///You are given a 0-indexed, strictly increasing integer array nums and a positive integer diff. A triplet (i, j, k) is an arithmetic triplet if the following conditions are met:
    ///
    /// i < j < k,
    /// nums[j] - nums[i] == diff, and
    /// nums[k] - nums[j] == diff.
    /// Return the number of unique arithmetic triplets.
    ///
    /// *Constraints*:
    ///
    /// 3 <= nums.length <= 200
    /// 0 <= nums[i] <= 200
    /// 1 <= diff <= 50
    /// nums is strictly increasing.
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut count = 0;
        let mut j = 0;
        let mut k = 0;

        for i in 0..nums.len() {
            while j < nums.len() && nums[j] < nums[i] + diff {
                j += 1;
            }

            while k < nums.len() && nums[k] < nums[i] + 2 * diff {
                k += 1;
            }

            if j < nums.len()
                && k < nums.len()
                && nums[j] == nums[i] + diff
                && nums[k] == nums[i] + 2 * diff
            {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }
}
