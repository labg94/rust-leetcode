struct Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let digit_sum: i32 = nums
            .iter()
            .map(|&num| Solution::calculate_digit_sum(num))
            .sum();

        sum - digit_sum
    }

    fn calculate_digit_sum(num: i32) -> i32 {
        let mut response = 0;
        let mut num = num;
        while num > 0 {
            response += num % 10;
            num /= 10;
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case_1() {
        let actual = Solution::difference_of_sum(vec![1, 15, 6, 3]);
        assert_eq!(actual, 9);
    }

    #[test]
    pub fn test_case_2() {
        let actual = Solution::difference_of_sum(vec![1, 2, 3, 4]);
        assert_eq!(actual, 0);
    }
}
