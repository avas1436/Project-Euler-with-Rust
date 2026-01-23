// problem number 25 of euler project
// 1000-digit Fibonacci Number

fn main() {
    // let ans: u128 = answare(1000);
    println! {"the answare is : "};
}

// fn fibonacci_sequence_contain_digits(digits: usize) -> usize {
//     let index: usize = digits;
//     println!("ok");
//     index
// }

fn sum_strings(a: &str, b: &str) -> String {
    let mut carry = 0;
    let mut result: String = String::new();
    let mut iter_a = a.chars().rev();
    let mut iter_b = b.chars().rev();

    let limit = b.len();
    let mut step = 0;

    loop {
        let da = match iter_a.next() {
            Some(c) => c.to_digit(10).unwrap(),
            None => 0,
        };
        let db = iter_b.next().map_or(0, |c| c.to_digit(10).unwrap());

        if step >= limit && db == 0 && da == 0 && carry == 0 {
            break;
        }

        let sum = da + db + carry;
        result.push(char::from_digit(sum % 10, 10).unwrap());
        carry = sum / 10;

        step += 1;
    }
    if carry != 0 {
        result.push(char::from_digit(carry, 10).unwrap());
    }

    result.chars().rev().collect()
}

// test
#[cfg(test)]
mod tests {
    // use crate::fibonacci_sequence_contain_digits;
    use crate::sum_strings;

    #[test]
    fn sum_1() {
        let result = sum_strings("1", "1");
        println!("Result of 1 + 1 is : {}", result);
        assert_eq!(result, "2");
    }

    #[test]
    fn sum_8_5() {
        let result = sum_strings("5", "8");
        println!("Result of 5 + 8 is : {}", result);
        assert_eq!(result, "13");
    }

    #[test]
    fn sum_2_digit() {
        let result = sum_strings("55", "89");
        println!("Result of 55 + 89 is : {}", result);
        assert_eq!(result, "144");
    }

    #[test]
    fn sum_dif_nums() {
        let result = sum_strings("9", "98");
        println!("Result of 9 + 98 is : {}", result);
        assert_eq!(result, "107");
    }

    #[test]
    fn zero_test() {
        let result = sum_strings("0", "0");
        println!("Result of 0 + 0 is : {}", result);
        assert_eq!(result, "0");
    }
}
