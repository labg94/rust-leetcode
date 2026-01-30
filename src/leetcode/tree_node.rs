use std::cell::RefCell;
use std::collections::VecDeque;
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
            right: None
        }
    }

    pub fn array_to_tree(arr: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
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

}

