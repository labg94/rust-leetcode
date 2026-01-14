struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let a = edges[0][0];
        let b = edges[0][1];
        if a == edges[1][0] || a == edges[1][1] {
            a
        } else {
            b
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_1() {
        assert_eq!(Solution::find_center(vec![vec![1,2],vec![2,3],vec![4,2]]), 2)   
    }


    #[test]
    fn test_case_2() {
        assert_eq!(Solution::find_center(vec![vec![1,2],vec![5,1],vec![1,3],vec![1,4]]), 1)
    }
} 