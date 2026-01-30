use std::cell::RefCell;
use std::rc::Rc;
use crate::leetcode::tree_node::TreeNode;

struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Self::dfs(root, low, high)
    }
    
    
    fn dfs(node :  Option<Rc<RefCell<TreeNode>>>,  low: i32, high: i32) -> i32{
        if let Some(node) = node {
            let nod = node.borrow(); 
            let sum = if nod.val>=low && nod.val <= high{
                nod.val
            }else{
                0
            };
            
          sum +  Self::dfs(nod.right.clone(),  low, high) + Self::dfs(nod.left.clone(), low, high)
            
        }else{
            0
        }
        
        
        
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_binary_tree() {
        let tree = TreeNode::array_to_tree(&vec![Some(10), Some(5), Some(15), Some(3), Some(7)]);
        assert!(tree.is_some());
        let root = tree.unwrap();
        assert_eq!(root.borrow().val, 10);
        assert_eq!(root.borrow().left.as_ref().unwrap().borrow().val, 5);
        assert_eq!(root.borrow().right.as_ref().unwrap().borrow().val, 15);
    }
    
    #[test]
    fn test_case_1(){
        let option = TreeNode::array_to_tree(&vec![Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]);

        let result = Solution::range_sum_bst(option, 7, 15);
        
        assert_eq!(result,32)
    }
    
    #[test]
    fn test_case_2(){
        let option = TreeNode::array_to_tree(&vec![Some(10),Some(5),Some(15),Some(3),Some(7),Some(13),Some(18),Some(1),None,Some(6)]);

        let result = Solution::range_sum_bst(option, 6, 10);
        
        assert_eq!(result,23)
    }
    
}