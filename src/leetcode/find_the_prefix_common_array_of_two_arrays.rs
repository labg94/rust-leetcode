struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        a.into_iter()
            .zip(b)
            .scan((0u64, 0u64), |st, (x, y)| {
                st.0 |= 1 << x;
                st.1 |= 1 << y;
                Some((st.0 & st.1).count_ones() as i32)
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_1() {
        assert_eq!(Solution::find_the_prefix_common_array(vec![1,3,2,4], vec![3,1,2,4]), vec![0,2,3,4])
    }
        
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::find_the_prefix_common_array(vec![2,3,1], vec![3,1,2]), vec![0,1,3])
    }
    
}
