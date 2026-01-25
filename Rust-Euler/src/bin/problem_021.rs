/*
    Project Euler - Problem 21
    Amicable Numbers

    This program finds the sum of all amicable numbers under a given limit.

    Two numbers a and b are considered amicable if:
    - The sum of proper divisors of a equals b
    - The sum of proper divisors of b equals a
    - a != b

    For example:
        220 -> 284
        284 -> 220
*/

use std::collections::HashMap;

fn main() {
    todo!("solve problem here");
}

struct AmicableNumbers {
    cache: HashMap<usize, usize>,
}

impl AmicableNumbers {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    fn sum_of_proper_divisors(number: usize) {
        todo!("this function should calculate sum of divisors of a number");
    }

    fn is_amicable(am: usize) -> bool {
        todo!("this function determine amicable numbers");
    }

    fn sum_amicables(limit: usize) -> usize {
        todo!("this function iterate from 1 to limit and sum all amicable numbers");
    }
}
