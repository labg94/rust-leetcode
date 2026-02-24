trait KeyChar{
    fn index(&self) -> usize;
}


impl KeyChar for char {
    fn index(&self) -> usize {
        (*self as u8 - b'a') as usize
    }
}

struct Solution;


impl Solution {
    ///You are given the strings key and message, which represent a cipher key and a secret message, respectively. The steps to decode message are as follows:
    ///
    /// 1. Use the first appearance of all 26 lowercase English letters in key as the order of the substitution table.
    /// 2. Align the substitution table with the regular English alphabet.
    /// 3. Each letter in message is then substituted using the table.
    /// 4. Spaces ' ' are transformed to themselves.
    /// - For example, given key = "happy boy" (actual key would have at least one instance of each letter in the alphabet), we have the partial substitution table of ('h' -> 'a', 'a' -> 'b', 'p' -> 'c', 'y' -> 'd', 'b' -> 'e', 'o' -> 'f').
    /// Return the decoded message.
    ///
    /// *Constraints:*
    ///
    /// - 26 <= key.length <= 2000
    /// - key consists of lowercase English letters and ' '.
    /// - key contains every letter in the English alphabet ('a' to 'z') at least once.
    /// - 1 <= message.length <= 2000
    /// - message consists of lowercase English letters and ' '.
    pub fn decode_message(key: String, message: String) -> String {
        let mut map = [None;26];
        let mut i = 0u8;

        for c in key.chars() {
            if c.is_whitespace() || map[c.index()].is_some() { continue; }
            map[c.index()] = Some((b'a' +i) as char);
            i += 1;
        }

        message.chars().map(|c| if c.is_whitespace() {c} else { map[c.index()].unwrap()}).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_1() {
        let actual = Solution::decode_message("the quick brown fox jumps over the lazy dog".to_string(), "vkbs bs t suepuv".to_string());
        assert_eq!(actual, "this is a secret");
    }

    #[test]
    fn test_case_2() {
        let actual = Solution::decode_message("eljuxhpwnyrdgtqkviszcfmabo".to_string(), "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string());
        assert_eq!(actual, "the five boxing wizards jump quickly");
    }
}