struct Solution;

impl Solution {

    ///You are given a 0-indexed integer array nums and an integer k.
    ///
    /// Return an integer that denotes the sum of elements in nums whose corresponding indices have exactly k set bits in their binary representation.
    ///
    /// The set bits in an integer are the 1's present when it is written in binary.
    ///
    /// For example, the binary representation of 21 is 10101, which has 3 set bits.
    ///
    /// Constraints:
    ///
    /// 1 <= nums.length <= 1000
    ///
    /// 1 <= nums\[i] <= 105
    ///
    /// 0 <= k <= 10
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut response = 0;

        for i in 0..nums.len() {
            if i.count_ones() == k as u32{
                response += nums[i];
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![5,10,1,5,2], 1), 13);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![4,3,2,1], 2), 1);
    }
}