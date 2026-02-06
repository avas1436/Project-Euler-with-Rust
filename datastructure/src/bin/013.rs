/*
Q3. Masking Personal Information

You are given a personal information string s, representing either an email address
or a phone number. Return the masked personal information using the below rules.

Email address:

An email address is:

    A name consisting of uppercase and lowercase English letters, followed by
    The '@' symbol, followed by
    The domain consisting of uppercase and lowercase English letters with a dot '.'
    somewhere in the middle (not the first or last character).

To mask an email:

    The uppercase letters in the name and domain must be converted to lowercase letters.
    The middle letters of the name (i.e., all but the first and last letters) must be
    replaced by 5 asterisks "*****".

Phone number:

A phone number is formatted as follows:

    The phone number contains 10-13 digits.
    The last 10 digits make up the local number.
    The remaining 0-3 digits, in the beginning, make up the country code.
    Separation characters from the set {'+', '-', '(', ')', ' '} separate the above
    digits in some way.

To mask a phone number:

    Remove all separation characters.
    The masked phone number should have the form:
        "***-***-XXXX" if the country code has 0 digits.
        "+*-***-***-XXXX" if the country code has 1 digit.
        "+**-***-***-XXXX" if the country code has 2 digits.
        "+***-***-***-XXXX" if the country code has 3 digits.
    "XXXX" is the last 4 digits of the local number.
 */

fn main() {
    let email = "abas.zade@outlook.com".to_string();
    let masked_email = Solution::mask_pii(email);
    println!("Masked Email: {}", masked_email);
}

struct Solution;

impl Solution {
    fn mask_email(email: String) -> String {
        let parts: Vec<&str> = email.split('@').collect();
        let name: Vec<char> = parts[0].to_lowercase().chars().collect();
        let domain = parts[1].to_lowercase();
        format!("{}*****{}@{}", name[0], name.last().unwrap(), domain).to_string()
    }

    fn mask_phone(phone: String) -> String {
        let phone_number: Vec<char> = phone.chars().rev().filter(|n| n.is_numeric()).collect();
        let local: Vec<char> = phone_number[0..4].iter().rev().cloned().collect();
        let country: Vec<char> = phone_number[10..].iter().rev().cloned().collect();
        let country_code = country.len();
        match country_code {
            0 => format!("***-***-{}", local.iter().collect::<String>()),
            1 => format!("+*-***-***-{}", local.iter().collect::<String>()),
            2 => format!("+**-***-***-{}", local.iter().collect::<String>()),
            3 => format!("+***-***-***-{}", local.iter().collect::<String>()),
            _ => format!("+****-***-***-{}", local.iter().collect::<String>()),
        }
    }

    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            return Self::mask_email(s);
        } else {
            return Self::mask_phone(s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_email() {
        assert_eq!(
            Solution::mask_pii("LeetCode@LeetCode.com".to_string()),
            "l*****e@leetcode.com"
        );
    }

    #[test]
    fn fake_email() {
        assert_eq!(
            Solution::mask_pii("AB@qq.com".to_string()),
            "a*****b@qq.com"
        );
    }

    #[test]
    fn long_email() {
        assert_eq!(
            Solution::mask_pii("abas.zade@outlook.com".to_string()),
            "a*****e@outlook.com"
        );
    }

    #[test]
    fn us_phone_number() {
        assert_eq!(
            Solution::mask_pii("1(234)567-890".to_string()),
            "***-***-7890"
        );
    }

    #[test]
    fn iran_phone_number() {
        assert_eq!(
            Solution::mask_pii("86-(10)12345678".to_string()),
            "+**-***-***-5678"
        );
    }

    #[test]
    fn china_phone_number() {
        assert_eq!(
            Solution::mask_pii("86-(10)12345678".to_string()),
            "+**-***-***-5678"
        );
    }

    #[test]
    fn japan_phone_number() {
        assert_eq!(
            Solution::mask_pii("86-(10)12345678".to_string()),
            "+**-***-***-5678"
        );
    }
}
