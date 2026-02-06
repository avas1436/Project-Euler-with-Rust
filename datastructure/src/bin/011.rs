/*
Q1. Detect Capital

We define the usage of capitals in a word to be right when one of the following
cases holds:

    All letters in this word are capitals, like "USA".
    All letters in this word are not capitals, like "leetcode".
    Only the first letter in this word is capital, like "Google".

Given a string word, return true if the usage of capitals in it is right.
 */
fn main() {
    let word = "leetcode".to_string();
    println!("{}", Solution::detect_capital_use(word));
}

struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut chars = word.chars();
        if word.to_uppercase() == word || word.to_lowercase() == word {
            return true;
        };
        let first = chars.next().unwrap();
        if first.is_uppercase() && chars.all(|c| c.is_lowercase()) {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_USA() {
        assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
    }
    #[test]
    fn detect_FlaG() {
        assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
    }

    #[test]
    fn detect_Google() {
        assert_eq!(Solution::detect_capital_use("Google".to_string()), true);
    }

    #[test]
    fn detect_Leetcode() {
        assert_eq!(Solution::detect_capital_use("Leetcode".to_string()), true);
    }
}
