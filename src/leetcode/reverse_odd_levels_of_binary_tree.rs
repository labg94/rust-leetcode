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
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            Solution::traverse_dfs(left, right, 0);
        }
        root
    }


    fn traverse_dfs(left:  Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>, level : u32){
        if let (Some(left), Some(right)) = (left.as_ref(), right.as_ref()) {

            if level % 2 == 0 {
                let left_val = left.borrow().val;
                let right_val = right.borrow().val;

                left.borrow_mut().val = right_val;
                right.borrow_mut().val = left_val;
            }

            Self::traverse_dfs(left.borrow().left.clone(), right.borrow().right.clone(), level+1);
            Self::traverse_dfs(left.borrow().right.clone(), right.borrow().left.clone(), level+1);

        }

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
        let input = array_to_tree([2, 3, 5, 8, 13, 21, 34].map(|n| Some(n)).as_slice());
        let response = array_to_tree([2, 5, 3, 8, 13, 21, 34].map(|n| Some(n)).as_slice());

        assert_eq!(Solution::reverse_odd_levels(input), response);
    }

    #[test]
    fn test_case_2() {
        let input = array_to_tree([7,13,11].map(|n| Some(n)).as_slice());
        let response = array_to_tree([7,11,13].map(|n| Some(n)).as_slice());

        assert_eq!(Solution::reverse_odd_levels(input), response);
    }

    #[test]
    fn test_case_3() {
        let input = array_to_tree([0,1,2,0,0,0,0,1,1,1,1,2,2,2,2].map(|n| Some(n)).as_slice());
        let response = array_to_tree([0,2,1,0,0,0,0,2,2,2,2,1,1,1,1].map(|n| Some(n)).as_slice());

        assert_eq!(Solution::reverse_odd_levels(input), response);
    }
}
