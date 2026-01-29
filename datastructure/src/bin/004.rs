// quiz
// Q1. Remove Duplicate Letters
//
// Given a string s, remove duplicate letters so that every letter appears once and only
//  once. You must make sure your result is the smallest in lexicographical order among
// all possible results.

struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut answare: String = String::new();
        // let mut stack = Vec::new();

        for c in s.chars() {
            if answare.contains(c) {
                continue;
            }
            answare.push(c);
        }

        answare
    }
}

fn main() {
    let num1: String = "bcabc".to_string();
    let num2: String = "bcabc".to_string();
    let num3: String = "cbacdcbc".to_string();
    let num4: String = "cbacdcbc".to_string();
    for num in [num1, num2, num3, num4] {
        let result = Solution::remove_duplicate_letters(num);
        println!("Result: {}", result);
    }
}
