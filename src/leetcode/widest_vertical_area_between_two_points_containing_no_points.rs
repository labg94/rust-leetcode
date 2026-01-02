struct Solution;


impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        
        let mut max = 0;

        for w in points.windows(2) {
            println!("{:?}", w);
            max = max.max(w[1][0] - w[0][0]);
        }
        max
    }
}


#[cfg(test)]
mod tests {
    use crate::leetcode::widest_vertical_area_between_two_points_containing_no_points::Solution;

    #[test]
    fn test_case_1() {
        let input = vec![vec![8, 7], vec![9, 9], vec![7, 4]];
        
        assert_eq!(Solution::max_width_of_vertical_area(input), 1);
    }
    
    
    #[test]
    fn test_case_2() {
        let input = vec![vec![3,1], vec![9,0], vec![1,0], vec![1,4],vec![5,3], vec![8,8]];
        
        assert_eq!(Solution::max_width_of_vertical_area(input), 3);
    }
    
    
}