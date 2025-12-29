struct Solution;

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();

        let mut accumulator =0;
        for i in 0..students.len() {
            accumulator += (students[i] - seats[i]).abs();
        }
        accumulator
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test_case_1() {
        assert_eq!(Solution::min_moves_to_seat(vec![3,1,5], vec![2,7,4]), 4);
    }
        
    
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::min_moves_to_seat(vec![4,1,5,9], vec![1,3,2,6]), 7);
    }
       
    #[test]
    fn test_case_3() {
        assert_eq!(Solution::min_moves_to_seat(vec![2,2,6,6], vec![1,3,2,6]), 4);
    }
    
}
