use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // let mut response = vec![];
        // let mut index = 0;
        //
        // for i in 0..nums.len() {
        //     loop {
        //         if response.len() <= index{
        //             response.push(vec![]);
        //         }
        //
        //         if response[index].contains(&nums[i]){
        //             index += 1;
        //         }else{
        //             response[index].push(nums[i]);
        //             index = 0;
        //             break;
        //         }
        //     }
        //
        // }
        //
        //
        // response

        let mut used_count: HashMap<i32, usize> = HashMap::new();
        let mut res: Vec<Vec<i32>> = Vec::new();

        for x in nums {
            let k = used_count.entry(x).or_insert(0);
            if res.len() == *k {
                res.push(Vec::new());
            }
            res[*k].push(x);
            *k += 1;
        }

        res
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1(){
        let input = vec![1,3,4,1,2,3,1];
        let expected_output = vec![vec![1,3,4,2], vec![1,3], vec![1]];
        assert_eq!(Solution::find_matrix(input), expected_output);
    }
    #[test]
    fn test_case_2(){
        let input = vec![1,2,3,4];
        let expected_output =  vec![vec![1,2,3,4]]; // vec![vec![4,3,2,1]]; this is the output from leetcode but the order does not matter in the end
        assert_eq!(Solution::find_matrix(input), expected_output);
    }
}
