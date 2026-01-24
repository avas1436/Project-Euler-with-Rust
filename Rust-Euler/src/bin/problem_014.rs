// collatz sequence
// Solve problem in BDD method:
fn main() {}

// Feature: collatz sequence calculations.

// Scenario: Calculate length of the Collatz sequence for a given number
// Given: Given a starting number of 100
// When: i calculate the sequence length
// Then: the result should be 26
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    // ----------- World / Context -----------
    struct CollatzWorld {
        number: u128,
        cache: HashMap<u128, usize>,
        result: Option<usize>,
    }

    impl CollatzWorld {
        fn new() -> Self {
            Self {
                number: 0,
                cache: HashMap::new(),
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
            self.result = Some(collatz_length(self.number, &mut self.cache));
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
}
