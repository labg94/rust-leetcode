struct Solution;

impl Solution {
    ///Given an integer num, return the number of steps to reduce it to zero.
    ///
    /// In one step, if the current number is even, you have to divide it by 2, otherwise, you have to subtract 1 from it.
    ///
    /// *Constraints*:
    /// - 0 <= num <= 106
    pub fn number_of_steps(num: i32) -> i32 {
        // COmmon approach
        // let mut counter = 0;
        // let mut rest = num;
        //
        // while rest != 0 {
        //     if rest % 2 == 0 {
        //         rest /= 2;
        //     } else {
        //         rest -= 1;
        //     }
        //     counter += 1;
        // }
        //
        // counter
        // binary approach
        if num == 0 { return 0; }
        let n = num as u32;

        let leading_zeros = n.leading_zeros();

        let bit_len = 32 - leading_zeros;
        let ones = n.count_ones();
        (bit_len - 1 + ones) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case_1() {
        assert_eq!(Solution::number_of_steps(14), 6);
    }

    #[test]
    pub fn test_case_2() {
        assert_eq!(Solution::number_of_steps(8), 4);
    }

    #[test]
    pub fn test_case_3() {
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
