// Q1. Can Make Arithmetic Progression From Sequence

// A sequence of numbers is called an arithmetic progression if the difference between any
// two consecutive elements is the same.

// Given an array of numbers arr, return true if the array can be rearranged to form an
// arithmetic progression. Otherwise, return false.

// نمیخواهم داده را clone کنم و همزمان نمی توانم امضای تابع را هم تغییر دهم پس از روش ریاضی می روم.
struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let index = arr.len();
        let slice = &mut arr[0..index];
        slice.sort();
        let difference: i32 = slice[1] - slice[0];

        for i in 2..index {
            if slice[i] - slice[i - 1] != difference {
                return false;
            }
        }
        return true;
    }
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
