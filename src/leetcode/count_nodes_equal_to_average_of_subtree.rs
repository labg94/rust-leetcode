use crate::leetcode::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map(|node| Self::get_sum_and_nodes(node).0).or(Some(0)).unwrap()
    }

    fn get_sum_and_nodes(node: Rc<RefCell<TreeNode>>) -> (i32, i32, i32) {
        let (left_equal_nodes, left_sum, left_total_nodes) =
            if let Some(left) = node.borrow().left.clone() {
                Self::get_sum_and_nodes(left)
            } else {
                (0, 0, 0)
            };
        let (right_equal_nodes, right_sum, right_total_nodes) =
            if let Some(right) = node.borrow().right.clone() {
                Self::get_sum_and_nodes(right)
            } else {
                (0, 0, 0)
            };
        let sum = left_sum + right_sum + node.borrow().val;
        let total_nodes = left_total_nodes + right_total_nodes + 1;
        let equal_nodes = if node.borrow().val == sum / total_nodes {
            left_equal_nodes + right_equal_nodes + 1
        } else {
            left_equal_nodes + right_equal_nodes
        };
        (equal_nodes, sum, total_nodes)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = vec![Some(4), Some(8), Some(5), Some(0), Some(1), None, Some(6)];
        assert_eq!(Solution::average_of_subtree(TreeNode::array_to_tree(&input)), 5)
    }
    #[test]
    fn test_case_2() {
        let input = vec![Some(1)];
        assert_eq!(Solution::average_of_subtree(TreeNode::array_to_tree(&input)), 1)
    }

}
