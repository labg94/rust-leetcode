struct Solution;


impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut response = 0;

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                    if nums1[i] % (nums2[j] * k) ==0 {
                        response +=1;
                    }
            }
        }
        response
    }

}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_1(){
        let nums1 = vec![1,3,4];
        let nums2 = vec![1,3,4];
        let k = 1;
        let expected = 5;
        let result = Solution::number_of_pairs(nums1, nums2, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2(){
        let nums1 = vec![1,2,4,12];
        let nums2 = vec![2,4];
        let k = 3;
        let expected = 2;
        let result = Solution::number_of_pairs(nums1, nums2, k);
        assert_eq!(result, expected);
    }
}