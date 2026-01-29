// problem number 25 of euler project
// 1000-digit Fibonacci Number

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let ans = fibonacci_sequence_contain_digits(1000);
    let finish = start.elapsed();
    println!("Problem solved in {:?}", finish);
    println! {"The answare is : {}", ans};
}

fn fibonacci_sequence_contain_digits(digits: usize) -> usize {
    let mut a = "1".to_string();
    let mut b = "1".to_string();
    let mut index: usize = 2;
    loop {
        index += 1;
        (b, a) = (sum_strings(&a, &b), b);
        if b.len() == digits {
            println!("The first {}-digit fibonacci sequence is : {}", digits, b);
            break;
        }
    }
    index
}

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
    use crate::fibonacci_sequence_contain_digits;
    use crate::sum_strings;

    //////////////////////////////////////////////////
    /// sum_strings
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
    //////////////////////////////////////////////////
    //////////////////////////////////////////////////

    #[test]
    fn first_2_digit() {
        let result = fibonacci_sequence_contain_digits(2);
        println!("The first 2-digit fib-seq index is : {}", result);
        assert_eq!(result, 7);
    }

    #[test]
    fn first_3_digit() {
        let result = fibonacci_sequence_contain_digits(3);
        println!("The first 3-digit fib-seq index is : {}", result);
        assert_eq!(result, 12);
    }
}
