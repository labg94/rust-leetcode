struct Solution;

impl Solution {
    ///You are given a 0-indexed integer array nums and a positive integer k.
    ///
    /// You can apply the following operation on the array any number of times:
    ///
    /// Choose any element of the array and flip a bit in its binary representation. Flipping a bit means changing a 0 to 1 or vice versa.
    /// Return the minimum number of operations required to make the bitwise XOR of all elements of the final array equal to k.
    ///
    /// Note that you can flip leading zero bits in the binary representation of elements. For example, for the number (101)2 you can flip the fourth bit and obtain (1101)2.
    ///
    /// Constraints:
    ///
    /// 1 <= nums.length <= 105
    /// 0 <= nums[i] <= 106
    /// 0 <= k <= 106
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut xor = 0;
        for &num in nums.iter() {
            xor ^= num;
        }

        (xor ^ k).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::min_operations(vec![2, 1, 3, 4], 1), 2);
    }
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::min_operations(vec![2, 0, 2, 0], 0), 0);
    }
    #[test]
    fn test_case_3() {
        assert_eq!(Solution::min_operations(vec![4], 7), 2);
    }
}
