use crate::leetcode::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {

        if root.is_none() {
            return None;
        }
       
        // Self::dfs(&root, 0); // this is created by me and helped by Ai

        let mut sum = 0;
        Self::reverse_inorder(&root, &mut sum);  // this is fully Ai created

        root
    }


    //<editor-fold desc="created by me">
    // fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, adjust: i32) -> Rc<RefCell<TreeNode>> {
    //     if node.is_none() {
    //         return Rc::new(RefCell::new(TreeNode::new(adjust)));
    //     }
    // 
    //     let node_ref = node.as_ref().unwrap();
    // 
    //     // Process right subtree first
    //     let right_child = node_ref.borrow().right.clone();
    //     let r = Self::dfs(&right_child, adjust);
    // 
    //     // Find the leftmost node in the right subtree result
    //     let dfsl_result = Self::dfsl(&Some(r));
    // 
    //     // Update current node's value
    //     node_ref.borrow_mut().val += dfsl_result.borrow().val;
    // 
    //     // Process left subtree with updated node value
    //     let left_child = node_ref.borrow().left.clone();
    //     let current_val = node_ref.borrow().val;
    //     Self::dfs(&left_child, current_val);
    // 
    //     node_ref.clone()
    // 
    // }

    // fn dfsl(node: &Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
    //     if let Some(node_ref) = node {
    //         let left_child = node_ref.borrow().left.clone();
    //         if left_child.is_some() {
    //             return Self::dfsl(&left_child);
    //         }
    //         node_ref.clone()
    //     } else {
    //         // This shouldn't happen in the context of how it's called
    //         Rc::new(RefCell::new(TreeNode::new(0)))
    //     }
    // }
    //</editor-fold>


    //<editor-fold desc="help with leetcode submitions and Ai">
    fn reverse_inorder(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node_ref) = node {
            // Traverse right subtree first (larger values)
            let right_child = node_ref.borrow().right.clone();
            Self::reverse_inorder(&right_child, sum);

            // Process current node
            *sum += node_ref.borrow().val;
            node_ref.borrow_mut().val = *sum;

            // Traverse left subtree (smaller values)
            let left_child = node_ref.borrow().left.clone();
            Self::reverse_inorder(&left_child, sum);
        }
    }
    //</editor-fold>


}






#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::tree_node::TreeNode;


    #[test]
    fn case_1() {
        // Input BST: [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
        let input = vec![
            Some(4), Some(1), Some(6), Some(0), Some(2), Some(5), Some(7),
            None, None, None, Some(3), None, None, None, Some(8)
        ];

        // Expected GST: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
        let expected = vec![
            Some(30), Some(36), Some(21), Some(36), Some(35), Some(26), Some(15),
            None, None, None, Some(33), None, None, None, Some(8)
        ];

        let input_tree = TreeNode::array_to_tree(&input);
        let expected_tree = TreeNode::array_to_tree(&expected);

        let result = Solution::bst_to_gst(input_tree);

        assert_eq!(result, expected_tree);
    }




}
