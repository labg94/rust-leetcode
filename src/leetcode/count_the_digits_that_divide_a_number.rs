struct Solution;

impl Solution {
    ///Given an integer ``num``, return the number of digits in num that divide num.
    ///
    ///  An integer val divides nums if nums % val == 0.
    /// **Constraints**:
    ///
    /// - 1 <= num <= 109
    /// - num does not contain 0 as one of its digits.
    pub fn count_digits(num: i32) -> i32 {
        let mut rest = num;
        let mut count = 0;
        while rest > 0 {
            let digit = rest % 10;

            if num % digit ==0{
                count += 1;
            }

            rest /=10;
        }
        count
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::count_digits(7), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::count_digits(121), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::count_digits(1248), 4);
    }
}