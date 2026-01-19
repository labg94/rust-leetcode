struct Solution;

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {

        let mut response = Vec::new();

        for i in 1..height.len() {

            if height[i-1] > threshold{
                response.push(i as i32);
            }

        }

        response
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::stable_mountains(vec![1, 2, 3, 4, 5], 2), vec![ 3, 4]);
    }
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::stable_mountains(vec![10,1,10,1,10], 3), vec![ 1,3]);
    }
    #[test]
    fn test_case_3() {
        assert_eq!(Solution::stable_mountains(vec![10,1,10,1,10], 10), vec![]);
    }
}