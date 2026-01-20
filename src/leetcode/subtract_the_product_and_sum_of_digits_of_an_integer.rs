struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut sum = 0;
        let mut product = 1;
        let mut rest = n;

        loop {
            let digit = rest % 10;
            rest /= 10;

            sum += digit;
            product *= digit;

            if rest == 0 {
                break;
            }
        }

        product - sum
    }

    pub fn subtract_product_and_sum_ai(n: i32) -> i32 {
        let (sum, product) = std::iter::successors(Some(n), |&n| (n > 9).then_some(n / 10))
            .map(|n| n % 10)
            .fold((0, 1), |(s, p), digit| (s + digit, p * digit));

        product - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
    }
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }
    #[test]
    fn test_case_3() {
        assert_eq!(Solution::subtract_product_and_sum(705), -12);
    }
    #[test]
    fn test_case_1_ai() {
        assert_eq!(Solution::subtract_product_and_sum_ai(234), 15);
    }
    #[test]
    fn test_case_2_ai() {
        assert_eq!(Solution::subtract_product_and_sum_ai(4421), 21);
    }
    #[test]
    fn test_case_3_ai() {
        assert_eq!(Solution::subtract_product_and_sum_ai(705), -12);
    }
}
