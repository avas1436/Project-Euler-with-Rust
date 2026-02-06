/*
Q3. Repeated String Match

Given two strings a and b, return the minimum number of times you should repeat
string a so that string b is a substring of it. If it is impossible for b to
be a substring of a after repeating it, return -1.

Notice: string "abc" repeated 0 times is "", repeated 1 time is "abc" and repeated 2
times is "abcabc".
 */

fn main() {
    let a = "abc".to_string();
    let b = "abcabcabc".to_string();
    println!("{}", Solution::repeated_string_match(a, b));
}

struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        // این خط مشخص می کند حداقل تعداد تکراری که باید انجام شود تا b در a باشد
        let k = (b.len() + a.len() - 1) / a.len();

        if a.repeat(k).contains(&b) {
            k as i32
        } else if a.repeat(k + 1).contains(&b) {
            (k + 1) as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substring() {
        assert_eq!(
            Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()),
            3
        );
    }

    #[test]
    fn repeated_string_match() {
        assert_eq!(
            Solution::repeated_string_match("a".to_string(), "aa".to_string()),
            3
        );
    }

    #[test]
    fn rotate_string() {
        assert_eq!(
            Solution::repeated_string_match("abcde".to_string(), "abced".to_string()),
            1
        );
    }

    #[test]
    fn zero_rotate() {
        assert_eq!(
            Solution::repeated_string_match("abcde".to_string(), "".to_string()),
            0
        );
    }
}
