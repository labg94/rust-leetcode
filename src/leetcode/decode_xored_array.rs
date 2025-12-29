struct Solution;


impl Solution{
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut response = vec![];
        response.push( first);
        
        encoded.iter().enumerate().for_each(|(i, &v)| {
            response.push(v ^ response[i]) ;
        });
        
        response
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::decode(vec![1,2,3], 1), vec![1,0,2,1]);
    }
    #[test]
    fn test_case_2() {
        assert_eq!(Solution::decode(vec![6,2,7,3], 4), vec![4,2,0,7,4]);
    }
}