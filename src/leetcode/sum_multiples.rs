struct Solution;

impl Solution {

    ///Given a positive integer n, find the sum of all integers in the range [1, n] inclusive that are divisible by 3, 5, or 7.
    ///
    /// Return an integer denoting the sum of all numbers in the given range satisfying the constraint.
    pub fn sum_of_multiples(n: i32) -> i32 {
        (3..=n).filter(|i| i % 3==0 || i % 5==0 || i % 7==0).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_1() {
        assert_eq!(Solution::sum_of_multiples(7), 21);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::sum_of_multiples(10), 40);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::sum_of_multiples(9), 30);
    }
}