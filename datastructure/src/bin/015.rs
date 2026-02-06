/*
Q2. Rotate String

Given two strings s and goal, return true if and only if s can become goal after some
number of shifts on s.

A shift on s consists of moving the leftmost character of s to the rightmost position.

    For example, if s = "abcde", then it will be "bcdea" after one shift.

 */

fn main() {
    todo!("problem solve here");
}

struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let slen = s.len();
        if slen != goal.len() {
            return false;
        }
        let mut str: Vec<char> = s.chars().collect();
        let goal: Vec<char> = goal.chars().collect();
        for _ in 0..slen as usize {
            if str == goal {
                return true;
            }
            let c = str.remove(0);
            str.push(c);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();
        assert_eq!(Solution::rotate_string(s, goal), true);
    }

    #[test]
    fn test_rotate_string2() {
        let s = "abcde".to_string();
        let goal = "abced".to_string();
        assert_eq!(Solution::rotate_string(s, goal), false);
    }
}
