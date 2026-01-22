// problem number 25 of euler project
// 1000-digit Fibonacci Number

fn main() {
    // let ans: u128 = answare(1000);
    println! {"the answare is"};
}

// fn fibonacci_sequence_contain_digits(digits: usize) -> usize {
//     let index: usize = digits;
//     println!("ok");
//     index
// }

fn sum_strings(a: &str, b: &str) -> String {
    let mut carry = 0;
    let mut result: String = String::new();

    for i in (0..b.len()).rev() {
        let da: usize = a[i].to_digit(10);
        let db: usize = b[i].to_digit(10);
        let c = da + db;
        if c > 9 {
            carry = c - 10;
            result.push(carry.);
        } else {
            result.push(c);
        }
    }
    result.join("")
}

// test
#[cfg(test)]
mod tests {
    // use crate::fibonacci_sequence_contain_digits;
    use crate::sum_strings;

    #[test]
    fn sum_1() {
        let result = sum_strings("1", "1");
        assert_eq!(result, "2");
    }

    #[test]
    fn sum_8_5() {
        let result = sum_strings("5", "8");
        assert_eq!(result, "13");
    }

    #[test]
    fn sum_2_digit() {
        let result = sum_strings("55", "89");
        assert_eq!(result, "144");
    }

    #[test]
    fn sum_dif_nums() {
        let result = sum_strings("9", "98");
        assert_eq!(result, "107");
    }
}
