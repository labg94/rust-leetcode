struct Solution;

impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut  start: usize = 0;
        let mut  end: usize = nums.len()-1;
        let mut response: Vec<i32> = vec![0; nums.len()];
        
        for x in nums {
            let module = x % 2;
            if module ==0 {
                response[start] = module;
                start += 1;
            }else{
                response[end] = module;
                end -= 1;
            }
        }
        
        
        response
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let vec1 = Solution::transform_array(vec![4,3,2,1]);

        assert_eq!(vec1, vec![0,0,1,1]);
    }

    #[test]
    fn case_2() {
        let vec1 = Solution::transform_array(vec![1,5,1,4,2]);

        assert_eq!(vec1, vec![0,0,1,1,1]);
    }
    
}
