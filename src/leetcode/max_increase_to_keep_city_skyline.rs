struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut rows: Vec<i32> = vec![];
        let mut columns: Vec<i32> = vec![];

        let mut accum = 0;
        let mut colum_index = 0;
        for i in 0..grid.len(){
            rows.push(grid[i].iter().max().unwrap().clone());
            columns.push(grid.iter().map(|row| row[colum_index]).max().unwrap().clone());
            colum_index += 1;
        }

        colum_index = 0;
        for i in 0..grid.len(){
            for j in 0..grid[i].len() {
                let max = rows[i].min(columns[colum_index]);
              accum += max - grid[i][j];
              colum_index += 1;
            }
            colum_index = 0;
        }


        accum
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1(){
        let grid = vec![vec![3,0,8,4], vec![2,4,5,7], vec![9,2,6,3], vec![0,3,1,0]];
        let expected = 35;
        let result = Solution::max_increase_keeping_skyline(grid);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2(){
        let grid = vec![vec![0,0,0], vec![0,0,0], vec![0,0,0],];
        let expected = 0;
        let result = Solution::max_increase_keeping_skyline(grid);
        assert_eq!(result, expected);
    }

}