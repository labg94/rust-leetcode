struct Solution;

impl Solution {

    ///There is a class with m students and n exams. You are given a **0-indexed** ``m x n`` integer matrix score, where each row represents one student and ``score[i][j]`` denotes the score the ith student got in the jth exam. The matrix score contains distinct integers only.
    ///
    /// You are also given an integer ``k``. Sort the students (i.e., the rows of the matrix) by their scores in the k<sup>th</sup> (**0-indexed**) exam from the highest to the lowest.
    ///
    /// Return the matrix after sorting it.
    ///
    /// Constraints:
    ///
    /// - m == score.length
    /// - n == score\[i].length
    /// - 1 <= m, n <= 250
    /// - 1 <= score\[i]\[j] <= 105
    /// - score consists of distinct integers.
    /// - 0 <= k < n
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_by(|a, b| b[k as usize].cmp(&a[k as usize]));
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::sort_the_students(vec![vec![10,6,9,1], vec![7,5,11,2], vec![4,8,3,15]], 2), vec![vec![7,5,11,2], vec![10,6,9,1], vec![4,8,3,15]])
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::sort_the_students(vec![vec![3,4], vec![5,6]], 0), vec![vec![5,6], vec![3,4]])
    }

}
