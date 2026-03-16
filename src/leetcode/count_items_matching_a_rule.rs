struct Solution;

impl Solution {

    ///You are given an array items, where each items[i] = [typei, colori, namei] describes the type, color, and name of the ith item. You are also given a rule represented by two strings, ruleKey and ruleValue.
    ///
    /// The ith item is said to match the rule if one of the following is true:
    ///
    /// - ruleKey == "type" and ruleValue == typei.
    /// - ruleKey == "color" and ruleValue == colori.
    /// - ruleKey == "name" and ruleValue == namei.
    ///
    /// Return the number of items that match the given rule.
    ///
    /// *Constraints*:
    /// - 1 <= items.length <= 104
    /// - 1 <= typei.length, colori.length, namei.length, ruleValue.length <= 10
    /// - ruleKey is equal to either "type", "color", or "name".
    /// - All strings consist only of lowercase letters.
    ///
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let key = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => unreachable!("invalid rule_key"),
        };

        let rule_value = rule_value.as_str();

        items.iter()
            .filter(|item| item[key].as_str() == rule_value)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case_1() {
        let actual = Solution::count_matches(vec![vec!["phone".to_string(),"blue".to_string(),"pixel".to_string()],vec!["computer".to_string(),"silver".to_string(),"lenovo".to_string()],vec!["phone".to_string(),"gold".to_string(),"iphone".to_string()]], "color".to_string(), "silver".to_string());
        assert_eq!(actual, 1);
    }

    #[test]
    pub fn test_case_2() {
        let actual = Solution::count_matches(vec![vec!["phone".to_string(),"blue".to_string(),"pixel".to_string()],vec!["computer".to_string(),"silver".to_string(),"phone".to_string()],vec!["phone".to_string(),"gold".to_string(),"iphone".to_string()]], "type".to_string(), "phone".to_string());
        assert_eq!(actual, 2);
    }

}