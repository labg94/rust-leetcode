struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split("-")
            .into_iter()
            .map(|v| Self::convert_to_binary(v))
            .collect::<Vec<String>>()
            .join("-")
        

    }
    
    
    fn convert_to_binary(num: &str) -> String {
        
        let number : i32 = num.parse().unwrap();
        let binary = format!("{:b}", number);
        
         binary
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let response = Solution::convert_date_to_binary("2080-02-29".to_string());

        assert_eq!(response, "100000100000-10-11101");
    }
    
    #[test]
    fn case_2() {
        let response = Solution::convert_date_to_binary("1900-01-01".to_string());

        assert_eq!(response, "11101101100-1-1");
    }


}
