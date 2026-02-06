use std::cell::RefCell;
use std::rc::Rc;
use crate::leetcode::tree_node::TreeNode;

struct Solution;

impl Solution {
    /// Given the root of a binary tree, return the sum of values of nodes with an even-valued grandparent. If there are no nodes with an even-valued grandparent, return 0.
    ///
    /// A grandparent of a node is the parent of its parent if it exists.
    ///
    /// **Constraints**:
    ///
    /// - The number of nodes in the tree is in the range [1, 104].
    /// - 1 <= Node.val <= 100
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(root, false, false)
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, parent_is_even: bool, grandparent_is_even: bool) -> i32 {
        let Some(node_rc) = root else { return 0 };

        let (val, left, right) = {
            let node = node_rc.borrow();
            (node.val, node.left.clone(), node.right.clone())
        };

        let is_even = val % 2 == 0;

        let left_sum = Self::dfs(left, is_even, parent_is_even);
        let right_sum = Self::dfs(right, is_even, parent_is_even);

        let self_add = if grandparent_is_even { val } else { 0 };
        self_add + left_sum + right_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::sum_even_grandparent(TreeNode::array_to_tree(&vec![

            Some(6),Some(7),Some(8),Some(2),Some(7),Some(1),Some(3),Some(9),None,Some(1),Some(4),None,None,None,Some(5)


        ])), 18);
    }
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::sum_even_grandparent(TreeNode::array_to_tree(&vec![Some(1)])), 0);
    }

}