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
    fn sum_of_proper_divisors(&mut self, number: usize) -> usize {
        if number == 1 {
            return 0;
        }
        if self.cache.contains_key(&number) {
            return *self.cache.get(&number).unwrap();
        }
        let mut sum: usize = 0;
        let mut div: usize = 1;
        while div * div <= number {
            if number.is_multiple_of(div) {
                sum += div;
                if div != number / div && div != 1 {
                    sum += number / div;
                }
            }
            div += 1;
        }
        self.cache.insert(number, sum);
        sum
    }

    fn is_amicable(&mut self, am: usize) -> bool {
        let am_candidate = self.sum_of_proper_divisors(am);
        if am == am_candidate {
            false
        } else {
            let am_candidate_candidate = self.sum_of_proper_divisors(am_candidate);
            am_candidate_candidate == am
        }
    }

    fn sum_amicables(&mut self, limit: usize) -> usize {
        let mut sum = 0;
        for i in 1..limit {
            if self.is_amicable(i) {
                sum += i;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_220_divisors() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.sum_of_proper_divisors(220);
        assert_eq!(result, 284);
    }

    #[test]
    fn test_sum_of_284_divisors() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.sum_of_proper_divisors(284);
        assert_eq!(result, 220);
    }

    #[test]
    fn test_sum_of_1_divisors() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.sum_of_proper_divisors(1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_if_284_is_amicable() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.is_amicable(284);
        assert_eq!(result, true);
    }

    #[test]
    fn test_if_220_is_amicable() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.is_amicable(220);
        assert_eq!(result, true);
    }

    #[test]
    fn test_if_100_is_amicable() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.is_amicable(100);
        assert_eq!(result, false);
    }

    #[test]
    fn test_sum_of_amicables_for_1() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.sum_amicables(1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_of_amicables_for_2() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.sum_amicables(2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_of_amicables_for_10() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.sum_amicables(10);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_of_amicables_for_300() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.sum_amicables(300);
        assert_eq!(result, 504);
    }

    #[test]
    fn test_sum_of_amicables_for_284() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.sum_amicables(284);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_of_amicables_for_10_000() {
        let mut amicable = AmicableNumbers::new();
        let result = amicable.sum_amicables(10_000);
        assert_eq!(result, 31_626);
    }
}
