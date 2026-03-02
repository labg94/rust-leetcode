struct Solution;

impl Solution {
    ///Given an array of integers arr, and three integers a, b and c. You need to find the number of good triplets.
    ///
    /// A triplet (arr[i], arr[j], arr[k]) is good if the following conditions are true:
    ///
    /// 0 <= i < j < k < arr.length
    /// |arr[i] - arr[j]| <= a
    /// |arr[j] - arr[k]| <= b
    /// |arr[i] - arr[k]| <= c
    /// Where |x| denotes the absolute value of x.
    ///
    /// Return the number of good triplets.
    ///
    /// *Constraints:*
    ///
    /// - 3 <= arr.length <= 100
    /// - 0 <= arr[i] <= 1000
    /// - 0 <= a, b, c <= 1000
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let mut counter = 0;


        for i in 0..arr.len() - 2 {
            for j in i+1..arr.len() - 1 {
                if (arr[i] - arr[j]).abs() <= a{
                    for k in j+1..arr.len() {
                        if (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                            counter += 1;
                        }
                    }
                }
            }
        }
        counter
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test_case_1() {
          let actual = Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3);
          assert_eq!(actual, 4);
    }

    #[test]
    pub fn test_case_2() {
          let actual = Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1);
          assert_eq!(actual, 0);
    }
}