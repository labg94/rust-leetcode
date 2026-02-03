struct Solution;


impl Solution {

    ///We are given a list nums of integers representing a list compressed with run-length encoding.
    ///
    /// Consider each adjacent pair of elements [freq, val] = [nums[2*i], nums[2*i+1]] (with i >= 0).  For each such pair, there are freq elements with value val concatenated in a sublist. Concatenate all the sublists from left to right to generate the decompressed list.
    ///
    /// Return the decompressed list.
    ///
    /// Constraints:
    ///
    ///2 <= nums.length <= 100
    ///
    /// nums.length % 2 == 0
    ///
    /// 1 <= nums[i] <= 100
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        // interesting option from leetcode
        // ((0..nums.len()).step_by(2))
        //     .flat_map(|idx| vec![nums[idx+1]; nums[idx] as usize])
        //     .collect()

        let mut response = vec![];
        for i in (0..nums.len()).step_by(2) {

            let freq = nums[i];
            let val = nums[i+1];
            for _j in 0..freq {
                response.push(val);
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
     assert_eq!(Solution::decompress_rl_elist(vec![1,2,3,4]), vec![2,4,4,4])
    }

    #[test]
    fn test_case_2() {
     assert_eq!(Solution::decompress_rl_elist(vec![1,1,2,3]), vec![1,3,3])
    }
}
