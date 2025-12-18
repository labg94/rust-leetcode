use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new(); // A mapping to store numbers and their indices
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num; // Find the required number to reach the target
            if let Some(&index) = num_map.get(&complement) {
                return vec![index as i32, i as i32]; // Return indices of the complement and current number
            }
            num_map.insert(num, i); // Store the number with its index
        }
        vec![] // This line is never reached due to the problem guarantee
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let vec1 = Solution::two_sum(vec![2, 7, 11, 15], 9);

        assert_eq!(vec1, vec![0, 1]);
    }

    #[test]
    fn case_2() {
        let vec1 = Solution::two_sum(vec![3, 2, 4], 6);

        assert_eq!(vec1, vec![1, 2]);
    }

    #[test]
    fn case_3() {
        let vec1 = Solution::two_sum(vec![3, 3], 6);

        assert_eq!(vec1, vec![0, 1]);
    }
    #[test]
    fn case_4() {
        let vec1 = Solution::two_sum(vec![3, 2, 3], 6);

        assert_eq!(vec1, vec![0, 2]);
    }
}
