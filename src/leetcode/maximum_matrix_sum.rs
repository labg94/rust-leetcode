struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut total_sum = 0;
        let mut min_abs= i32::MAX;
        let mut negative_count = 0;    
        
        for r in matrix {
            for c in r {
                total_sum += c.abs() as i64;
                if c < 0{
                    negative_count +=1;
                }
                min_abs = min_abs.min(c.abs());
            }
            
        }
        if negative_count % 2 != 0 {
            total_sum -= 2 *min_abs as i64;
        }
        
        total_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);
    }
    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
            16
        );
    }
}
