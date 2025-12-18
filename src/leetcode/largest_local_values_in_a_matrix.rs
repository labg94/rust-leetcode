struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut response_grid: Vec<Vec<i32>> = Vec::new();


        for i in 0..3 {
            println!("{:?}", i);
        }

        let n = grid.len() - 2;
        for i in 0..n {
            let mut row = Vec::new();
            for j in 0..n {
                row.push(Self::find_max(&grid, i as i32, j as i32));
            }
            response_grid.push(row);
        }

        response_grid
    }
    
    
    
    
    fn find_max(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> i32{
        let mut max = 0;

        for i in x..x+3 {
            for j in y..y+3 {
                max = max.max(grid[i as usize][j as usize]);
            }
        }
        
        
        
        max        
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let response = Solution::largest_local(vec![vec![9,9,8,1],vec![5,6,2,6],vec![8,2,6,4], vec![6,2,2,2]]);
        
        assert_eq!(response, vec![vec![9,9], vec![8,6]]);
    }



}
