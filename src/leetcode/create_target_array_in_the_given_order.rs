struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut response = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            response.insert(index[i] as usize, nums[i]);
        }

        response
    }

    pub fn create_target_array_2(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut response = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            response.push(nums[index[i] as usize]);


            for j in (index[i] as usize + 1..response.len()).rev() {
                response[j] = response[j-1]
            }

            response[index[i] as usize] = nums[i];

        }



        response
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1(){
        assert_eq!(Solution::create_target_array(vec![0,1,2,3,4], vec![0,1,2,2,1]), vec![0,4,1,3,2]);
        assert_eq!(Solution::create_target_array_2(vec![0,1,2,3,4], vec![0,1,2,2,1]), vec![0,4,1,3,2])
    }

    #[test]
    fn test_case_2(){
        assert_eq!(Solution::create_target_array(vec![1,2,3,4,0], vec![0,1,2,3,0]), vec![0,1,2,3,4]);
        assert_eq!(Solution::create_target_array_2(vec![1,2,3,4,0], vec![0,1,2,3,0]), vec![0,1,2,3,4])
    }

    #[test]
    fn test_case_3(){
        assert_eq!(Solution::create_target_array(vec![1], vec![0]), vec![1])
    }

}