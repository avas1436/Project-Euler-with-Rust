use std::collections::HashMap;

// collatz sequence
// Solve problem in BDD method:
fn main() {}

struct ColatzCalculator {
    cache: HashMap<u128, usize>,
    start: u128,
    end: u128,
}

impl ColatzCalculator {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            start: 1,
            end: 2_000_000,
        }
    }
    fn collatz_length(&mut self, number: u128) -> usize {
        // calculate the collatz sequence length
        let mut num = number;
        let mut step: usize = 1;
        while num != 1 {
            if let Some(&length) = self.cache.get(&num) {
                let total = step + length;
                self.cache.insert(number, total);
                return total;
            }
            if num % 2 == 0 {
                num /= 2;
            } else {
                num = (3 * num) + 1;
            }
            step += 1;
        }
        self.cache.insert(number, step);
        step
    }

    fn max_collatz_range(&mut self, start: u128, end: u128) -> usize {
        // calculate the maximum of the collatz sequence length
        let mut max_leng: usize = 0;
        let mut num_max_leng: u128 = 0;
        let mut num = start;
        while num < end {
            let leng = self.collatz_length(num);
            if leng > max_leng {
                max_leng = leng;
                num_max_leng = num;
            }
            num += 1;
        }
        println!(
            "Max collatz sequence bellow {end} is for number {num_max_leng} and its length is : {max_leng}"
        );
        max_leng
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
        number: u128,
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
        fn given_a_starting_number(mut self, number: u128) -> Self {
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
}
