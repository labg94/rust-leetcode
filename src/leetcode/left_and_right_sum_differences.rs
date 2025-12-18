struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut response = vec![];
        let mut left = vec![];
        let mut total = 0;
        for i in 0..nums.len() {
            if i == 0 {
                left.push(0);
            } else {
                left.push(nums[i - 1] + left[i-1]);
            }
            total += nums[i];
        }

        for i in 0..nums.len() {
            total -= nums[i];
            let abs_val = (left[i] - total).abs();
            response.push(abs_val);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let response = Solution::left_right_difference(vec![10, 4, 8, 3]);
        assert_eq!(response, vec![15, 1, 11, 22]);
    }  
    
    #[test]
    fn case_2() {
        let response = Solution::left_right_difference(vec![1]);
        assert_eq!(response, vec![0]);
    }
}
