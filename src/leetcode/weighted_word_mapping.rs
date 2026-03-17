struct Solution;


impl Solution {

    ///You are given an array of strings words, where each string represents a word containing lowercase English letters.
    ///
    /// You are also given an integer array weights of length 26, where weights[i] represents the weight of the ith lowercase English letter.
    ///
    /// The weight of a word is defined as the sum of the weights of its characters.
    ///
    /// For each word, take its weight modulo 26 and map the result to a lowercase English letter using reverse alphabetical order (0 -> 'z', 1 -> 'y', ..., 25 -> 'a').
    ///
    /// Return a string formed by concatenating the mapped characters for all words in order.
    ///
    /// *Constraints:*
    ///
    /// - 1 <= words.length <= 100
    /// - 1 <= words[i].length <= 10
    /// - weights.length == 26
    /// - 1 <= weights[i] <= 100
    /// - words[i] consists of lowercase English letters.
    ///
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut response = String::with_capacity(words.len());

        for word in words {
            let sum: i32 = word.bytes().map(|b| weights[(b - b'a') as usize]).sum();

            let module = sum % 26;

            let char = ('z' as u8 - module  as u8) as char;

            response.push(char);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let actual = Solution::map_word_weights(vec!["abcd","def","xyz"].iter().map(|s| s.to_string()).collect(), vec![5,3,12,14,1,2,3,2,10,6,6,9,7,8,7,10,8,9,6,9,9,8,3,7,7,2]);
        assert_eq!(actual, "rij");
    }

    #[test]
    fn test_case_2() {
        let actual = Solution::map_word_weights(vec!["a","b","c"].iter().map(|s| s.to_string()).collect(), vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]);
        assert_eq!(actual, "yyy");
    }

    #[test]
    fn test_case_3() {
        let actual = Solution::map_word_weights(vec!["abcd"].iter().map(|s| s.to_string()).collect(), vec![7,5,3,4,3,5,4,9,4,2,2,7,10,2,5,10,6,1,2,2,4,1,3,4,4,5]);
        assert_eq!(actual, "g");
    }
}