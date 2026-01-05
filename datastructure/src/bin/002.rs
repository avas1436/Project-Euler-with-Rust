// Q1. Can Make Arithmetic Progression From Sequence

// A sequence of numbers is called an arithmetic progression if the difference between any
// two consecutive elements is the same.

// Given an array of numbers arr, return true if the array can be rearranged to form an
// arithmetic progression. Otherwise, return false.

// نمیخواهم داده را clone کنم و همزمان نمی توانم امضای تابع را هم تغییر دهم پس از روش ریاضی می روم.
struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let length = arr.len() as i32;
        if length < 2 {
            return true;
        };
        let max = arr.iter().max().unwrap().clone();
        let min = arr.iter().min().unwrap().clone();
        let diff = (max - min) / (length - 1);
        if diff == 0 {
            return true;
        };
        for num in arr {
            if (num - min) / diff != 0 {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    let num1: Vec<i32> = vec![3, 5, 1];
    let num2: Vec<i32> = vec![1, 2, 4];
    let num3: Vec<i32> = vec![
        13, 12, -12, 9, 9, 16, 7, -10, -20, 0, 18, -1, -20, -10, -8, 15, 15, 16, 2, 15,
    ];
    for num in [num1, num2, num3] {
        let result = Solution::can_make_arithmetic_progression(num);
        println!("Result: {}", result);
    }
}
