use std::collections::HashMap;
use std::time::Instant;

// collatz sequence
// Solve problem in BDD method:
fn main() {
    let start = Instant::now();
    let mut answare = ColatzCalculator::new();
    let (number, length) = answare.max_collatz_range(1, 2_000_000);
    let end = start.elapsed();
    println!("Problem Solved In {:?}", end);
    println!(
        "The longest Collatz sequence below 2,000,000 starts at {} and has length {}.",
        number, length
    );
}

struct ColatzCalculator {
    cache: HashMap<usize, usize>,
}

impl ColatzCalculator {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    fn collatz_length(&mut self, number: usize) -> usize {
        // calculate the collatz sequence length
        let mut num = number;
        let mut step: usize = 1;
        while num != 1 {
            if let Some(&length) = self.cache.get(&num) {
                let total = step + length - 1;
                self.cache.insert(number, total);
                return total;
            }
            if num % 2 == 0 {
                num /= 2;
                step += 1;
            } else {
                num = ((3 * num) + 1) / 2;
                step += 2;
            }
        }
        self.cache.insert(number, step);
        step
    }

    fn max_collatz_range(&mut self, start: usize, end: usize) -> (usize, usize) {
        // calculate the maximum of the collatz sequence length
        let mut max_leng: usize = 0;
        let mut num_max_leng: usize = 0;
        let mut num = start;
        while num < end {
            let leng = self.collatz_length(num);
            if leng > max_leng {
                max_leng = leng;
                num_max_leng = num;
            }
            num += 1;
        }
        (num_max_leng, max_leng)
    }
}

// Feature: collatz sequence calculations.

// Scenario: Calculate length of the Collatz sequence for a given number
// Given: Given a starting number of 100
// When: i calculate the sequence length
// Then: the result should be 26
#[cfg(test)]
mod tests {
    use super::*;

    // ----------- World / Context -----------
    struct CollatzWorld {
        number: usize,
        calculator: ColatzCalculator,
        result: Option<usize>,
    }

    impl CollatzWorld {
        fn new() -> Self {
            Self {
                number: 0,
                calculator: ColatzCalculator::new(),
                result: None,
            }
        }

        // ----------- Given ----------------
        fn given_a_starting_number(mut self, number: usize) -> Self {
            self.number = number;
            self
        }

        // ----------- When ----------------
        fn when_i_calculate_the_sequence_length(mut self) -> Self {
            self.result = Some(self.calculator.collatz_length(self.number));
            self
        }

        // ----------- Then ----------------
        fn then_the_result_should_be(self, expected: usize) {
            let result = self.result.expect("When step was not executed");
            assert_eq!(result, expected);
        }
    }

    // ----------- Scenario ----------------
    #[test]
    fn calculate_length_of_the_collatz_sequence_for_100() {
        CollatzWorld::new()
            .given_a_starting_number(100)
            .when_i_calculate_the_sequence_length()
            .then_the_result_should_be(26);
    }

    #[test]
    fn calculate_length_of_the_collatz_sequence_for_1() {
        CollatzWorld::new()
            .given_a_starting_number(1)
            .when_i_calculate_the_sequence_length()
            .then_the_result_should_be(1);
    }

    #[test]
    fn calculate_length_of_the_collatz_sequence_for_even() {
        CollatzWorld::new()
            .given_a_starting_number(6)
            .when_i_calculate_the_sequence_length()
            .then_the_result_should_be(9);
    }

    #[test]
    fn calculate_length_of_the_collatz_sequence_for_odd() {
        CollatzWorld::new()
            .given_a_starting_number(7)
            .when_i_calculate_the_sequence_length()
            .then_the_result_should_be(17);
    }

    #[test]
    fn calculate_length_of_the_collatz_sequence_for_nine() {
        CollatzWorld::new()
            .given_a_starting_number(9)
            .when_i_calculate_the_sequence_length()
            .then_the_result_should_be(20);
    }

    #[test]
    fn max_collatz_in_small_range() {
        let mut calc = ColatzCalculator::new();
        let max_len = calc.max_collatz_range(1, 10);
        assert_eq!(max_len, (9, 20));
    }

    #[test]
    fn max_collatz_single_number_range() {
        let mut calc = ColatzCalculator::new();
        let max_len = calc.max_collatz_range(13, 14);
        assert_eq!(max_len, (13, 10));
    }

    #[test]
    fn max_collatz_non_one_start() {
        let mut calc = ColatzCalculator::new();
        let max_len = calc.max_collatz_range(10, 20);
        assert_eq!(max_len, (18, 21));
    }

    #[test]
    fn max_collatz_reuse_cache() {
        let mut calc = ColatzCalculator::new();

        let first = calc.max_collatz_range(1, 100_000);
        let second = calc.max_collatz_range(1, 100_000);

        assert_eq!(first, second);
    }

    #[test]
    fn max_collatz_reuse_cache_for_six() {
        let mut calc = ColatzCalculator::new();

        let first = calc.max_collatz_range(1, 6);
        let second = calc.max_collatz_range(1, 6);

        assert_eq!(first, second);
    }

    #[test]
    fn max_collatz_under_one_million() {
        let mut calc = ColatzCalculator::new();
        let max_len = calc.max_collatz_range(1, 1_000_000);
        assert_eq!(max_len, (837799, 525));
    }
}
