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
      right: None
    }
  }
}


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

    fn create_binary_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = vec![Rc::clone(&root)];
        let mut i = 1;

        while !queue.is_empty() && i < values.len() {
            let current = queue.remove(0);

            if i < values.len() {
                if let Some(val) = values[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    current.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push(left);
                }
            }
            i += 1;

            if i < values.len() {
                if let Some(val) = values[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    current.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push(right);
                }
            }
            i += 1;
        }

        Some(root)
    }

    #[test]
    fn test_create_binary_tree() {
        let tree = create_binary_tree(vec![Some(10), Some(5), Some(15), Some(3), Some(7)]);
        assert!(tree.is_some());
        let root = tree.unwrap();
        assert_eq!(root.borrow().val, 10);
        assert_eq!(root.borrow().left.as_ref().unwrap().borrow().val, 5);
        assert_eq!(root.borrow().right.as_ref().unwrap().borrow().val, 15);
    }
    
    #[test]
    fn test_case_1(){
        let option = create_binary_tree(vec![Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]);

        let result = Solution::range_sum_bst(option, 7, 15);
        
        assert_eq!(result,32)
    }
    
    #[test]
    fn test_case_2(){
        let option = create_binary_tree(vec![Some(10),Some(5),Some(15),Some(3),Some(7),Some(13),Some(18),Some(1),None,Some(6)]);

        let result = Solution::range_sum_bst(option, 6, 10);
        
        assert_eq!(result,23)
    }
    
}