struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut response: Vec<i32> = vec![0; nums.len()];
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j { continue; }
                if nums[i] > nums[j] {
                    response[i] +=1;
                }
                
            }
        }
        
        response
    }


    pub fn ai_smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut indexed_nums: Vec<(i32, usize)> = nums.iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect();

        indexed_nums.sort_unstable_by_key(|&(val, _)| val);

        let mut response = vec![0; nums.len()];
        let mut count = 0;

        for i in 0..indexed_nums.len() {
            let (current_val, original_index) = indexed_nums[i];

            // Skip duplicates when counting
            if i > 0 && indexed_nums[i - 1].0 < current_val {
                count = i as i32;
            }

            response[original_index] = count;
        }

        response
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let vec1 = Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]);
        
        assert_eq!(vec1, vec![4, 0, 1, 1, 3]);
    }
    
    #[test]
    fn case_2() {
        let vec1 = Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]);
        
        assert_eq!(vec1, vec![2, 1, 0, 3]);
    }
    
    #[test]
    fn case_1_ai() {
        let vec1 = Solution::ai_smaller_numbers_than_current(vec![8, 1, 2, 2, 3]);
        
        assert_eq!(vec1, vec![4, 0, 1, 1, 3]);
    }
    
    #[test]
    fn case_2_ai() {
        let vec1 = Solution::ai_smaller_numbers_than_current(vec![6, 5, 4, 8]);
        
        assert_eq!(vec1, vec![2, 1, 0, 3]);
    }
    
}