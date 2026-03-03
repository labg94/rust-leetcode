struct Solution;

impl Solution {

    ///You are given an integer array nums of size n. For each index i where 0 <= i < n, define a subarray nums[start ... i] where start = max(0, i - nums[i]).
    ///
    /// Return the total sum of all elements from the subarray defined for each index in the array.
    ///
    /// *Constraints:*
    ///
    /// - 1 <= n == nums.length <= 100
    /// - 1 <= nums[i] <= 1000
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // prefix[k] = nums[0] + ... + nums[k-1]
        let mut prefix = vec![0i32; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i];
        }


        println!("prefix {:?}", prefix);
        let mut response = 0i32;
        for i in 0..n {
            let start = (i as i32 - nums[i]).max(0) as usize;
            response += prefix[i + 1] - prefix[start];
        }
        println!("response {:?}", response);
        response
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case_1() {
        assert_eq!(Solution::subarray_sum(vec![2, 3, 1]), 11);
    }

    #[test]
    pub fn test_case_2() {
        assert_eq!(Solution::subarray_sum(vec![3,1,1,2]), 13);
    }
}