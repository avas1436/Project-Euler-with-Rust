/*
Q1. Repeated Substring Pattern

Given a string s, check if it can be constructed by taking a substring of it and
appending multiple copies of the substring together.
 */
fn main() {
    let s = "abab".to_string();
    println!("{}", Solution::repeated_substring_pattern(s));
}

struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        for i in 1..((s.len() / 2) + 1) {
            if s.len() % i == 0 {
                let substring = &s[0..i];
                if s == substring.repeat(s.len() / i) {
                    return true;
                }
            } else {
                continue;
            }
        }
        false
    }
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
