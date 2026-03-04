struct Solution;

impl Solution {
    ///You are given a 0-indexed integer array nums of even length and there is also an empty array arr. Alice and Bob decided to play a game where in every round Alice and Bob will do one move. The rules of the game are as follows:
    ///
    /// Every round, first Alice will remove the minimum element from nums, and then Bob does the same.
    /// Now, first Bob will append the removed element in the array arr, and then Alice does the same.
    /// The game continues until nums becomes empty.
    /// Return the resulting array arr.
    ///
    /// *Constraints*:
    ///
    /// - 2 <= nums.length <= 100
    /// - 1 <= nums[i] <= 100
    /// - nums.length % 2 == 0
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        for i in (0..nums.len()).step_by(2) {
            nums.swap(i, i + 1);
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::number_game(vec![5, 4, 2, 3]), vec![3, 2, 5, 4]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::number_game(vec![2, 5]), vec![5, 2]);
    }
}
