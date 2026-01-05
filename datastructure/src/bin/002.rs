// Q1. Can Make Arithmetic Progression From Sequence

// A sequence of numbers is called an arithmetic progression if the difference between any
// two consecutive elements is the same.

// Given an array of numbers arr, return true if the array can be rearranged to form an
// arithmetic progression. Otherwise, return false.

struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {}
}

fn main() {
    let num1: Vec<i32> = vec![3, 5, 1];
    let num2: Vec<i32> = vec![1, 2, 4];
    let num3: Vec<i32> = vec![1];
    for num in [num1, num2, num3] {
        let result = Solution::can_make_arithmetic_progression(num);
        println!("Result: {}", result);
    }
}
