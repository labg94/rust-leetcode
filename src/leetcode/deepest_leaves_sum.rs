use crate::leetcode::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {

    fn get_sum_max_level(node: Option<Rc<RefCell<TreeNode>>>, level: usize, sum: &mut i32, max_lvl: &mut usize) {
        if let Some(node) = node.as_ref() {
            let node = node.borrow();

            if level > *max_lvl {
                *sum = node.val;
                *max_lvl = level;
            } else if level == *max_lvl {
                *sum += node.val;
            }

            Self::get_sum_max_level(node.left.clone(), level + 1, sum, max_lvl);
            Self::get_sum_max_level(node.right.clone(), level + 1, sum, max_lvl);
        }
    }

    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::get_sum_max_level(root, 0, &mut sum, &mut 0);
        sum
    }


}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_1(){
        let arr = vec![Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), None, None, None, None, Some(8)];
        let root = TreeNode::array_to_tree(&arr);
        assert_eq!(Solution::deepest_leaves_sum(root), 15);
    }

    #[test]
    fn test_case_2(){
        let arr = vec![Some(6), Some(7), Some(8), Some(2), Some(7), Some(1), Some(3), Some(9),None, Some(1), Some(4), None, None, None, Some(5)];
        let root = TreeNode::array_to_tree(&arr);
        assert_eq!(Solution::deepest_leaves_sum(root), 19);
    }

}