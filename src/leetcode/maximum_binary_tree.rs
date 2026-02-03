use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::tree_node::TreeNode;

struct Solution;

impl Solution {

    ///
    /// You are given an integer array nums with no duplicates. A maximum binary tree can be built recursively from nums using the following algorithm:
    ///
    /// Create a root node whose value is the maximum value in nums.
    /// Recursively build the left subtree on the subarray prefix to the left of the maximum value.
    /// Recursively build the right subtree on the subarray suffix to the right of the maximum value.
    /// Return the maximum binary tree built from nums.
    ///
    /// Constraints:
    ///
    /// 1 <= nums.length <= 1000
    /// 0 <= nums[i] <= 1000
    /// All integers in nums are unique.
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::create_node(&nums)
    }

    fn create_node(nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }

        let max = nums.iter().enumerate().max_by_key(|(_, v)| **v).unwrap();

        let max_index = max.0;

        let mut node = TreeNode::new(max.1.clone());

        node.left = Self::create_node(&(&nums[..max_index].to_vec()));
        node.right = Self::create_node(&(&nums[max_index + 1..].to_vec()));

        let node = Rc::new(RefCell::new(node));

        Some(node)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let option = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
        let expected = TreeNode::array_to_tree(&vec![Some(6), Some(3), Some(5), None, Some(2), Some(0), None, None, Some(1)]);
        assert_eq!(option, expected);
    }

    #[test]
    fn test_case_2() {
        let option = Solution::construct_maximum_binary_tree(vec![3, 2, 1]);
        let expected = TreeNode::array_to_tree(&vec![Some(3),None,Some(2),None,Some(1)]);

        assert_eq!(option, expected);
    }

}