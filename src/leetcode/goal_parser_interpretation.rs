struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut response = String::new();
        let mut al_started = false;

        for c in command.chars() {
            if c == 'G' {
                response.push('G');
            }

            if c == 'a' {
                al_started = true;
            }

            if c == ')' {
                if al_started {
                    response.push_str("al");
                    al_started = false;
                } else {
                    response.push('o');
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
    fn case_1() {
        let string = Solution::interpret("G()(al)".to_string());

        assert_eq!(string, "Goal".to_string());
    }

    #[test]
    fn case_2() {
        let string = Solution::interpret("G()()()()(al)".to_string());

        assert_eq!(string, "Gooooal".to_string());
    }

    #[test]
    fn case_3() {
        let string = Solution::interpret("(al)G(al)()()G".to_string());

        assert_eq!(string, "alGalooG".to_string());
    }
}
