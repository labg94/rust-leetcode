use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
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
    use std::collections::VecDeque;

    fn array_to_tree(arr: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.is_empty() || arr[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(arr[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while !queue.is_empty() && i < arr.len() {
            let node = queue.pop_front().unwrap();

            // Add left child
            if i < arr.len() {
                if let Some(val) = arr[i] {
                    let left_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left_child.clone());
                    queue.push_back(left_child);
                }
                i += 1;
            }

            // Add right child
            if i < arr.len() {
                if let Some(val) = arr[i] {
                    let right_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(right_child.clone());
                    queue.push_back(right_child);
                }
                i += 1;
            }
        }

        Some(root)
    }

    #[test]
    fn test_case_1() {
        let input = vec![Some(4), Some(8), Some(5), Some(0), Some(1), None, Some(6)];
        assert_eq!(Solution::average_of_subtree(array_to_tree(&input)), 5)
    }
    #[test]
    fn test_case_2() {
        let input = vec![Some(1)];
        assert_eq!(Solution::average_of_subtree(array_to_tree(&input)), 1)
    }

}
