/*
Q1. Repeated Substring Pattern

Given a string s, check if it can be constructed by taking a substring of it and
appending multiple copies of the substring together.
 */
fn main() {
    todo!("problem solve here");
}

struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ab() {
        assert_eq!(
            Solution::repeated_substring_pattern("abab".to_string()),
            true
        );
    }

    #[test]
    fn test_ababab() {
        assert_eq!(
            Solution::repeated_substring_pattern("aba".to_string()),
            false
        );
    }

    #[test]
    fn test_abababab() {
        assert_eq!(
            Solution::repeated_substring_pattern("abcabcabcabc".to_string()),
            true
        );
    }
}
