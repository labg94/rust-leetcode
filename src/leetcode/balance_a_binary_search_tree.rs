use crate::leetcode::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    ///Given the root of a binary search tree, return a balanced binary search tree with the same node values. If there is more than one answer, return any of them.
    ///
    /// A binary search tree is balanced if the depth of the two subtrees of every node never differs by more than 1.
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            if let Some(rc) = node {
                let n = rc.borrow();
                inorder(&n.left, vals);
                vals.push(n.val);
                inorder(&n.right, vals);
            }
        }

        fn build(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if vals.is_empty() {
                return None;
            }
            let mid = vals.len() / 2;

            let left = build(&vals[..mid]);
            let right = build(&vals[mid + 1..]);

            Some(Rc::new(RefCell::new(TreeNode {
                val: vals[mid],
                left,
                right,
            })))
        }

        let mut vals = Vec::new();
        inorder(&root, &mut vals);
        build(&vals)
    }



}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let given = TreeNode::array_to_tree(&vec![Some(1),None,Some(2),None,Some(3),None,Some(4),None,None]);

        let actual = Solution::balance_bst(given);

        let expected = TreeNode::array_to_tree(&vec![Some(3),Some(2),Some(4),Some(1)]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case_2() {
        let given = TreeNode::array_to_tree(&vec![Some(2),Some(1), Some(3)]);
        let actual = Solution::balance_bst(given);
        let expected = TreeNode::array_to_tree(&vec![Some(2),Some(1), Some(3)]);
        assert_eq!(actual, expected);
    }

}