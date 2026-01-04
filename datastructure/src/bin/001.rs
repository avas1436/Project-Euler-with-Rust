use std::cmp::min;

struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut maximum: i32 = 0;

        for i in 0..(heights.len() - 1) {
            let min = min(heights[i], heights[i + 1]);
            let area = min * 2;

            if area > maximum {
                maximum = area;
            }
        }

        maximum
    }
}

fn main() {
    let num1: Vec<i32> = vec![2, 1, 5, 6, 2, 3];
    let num2: Vec<i32> = vec![2, 4];
    let num3: Vec<i32> = vec![1];
    for num in [num1, num2, num3] {
        let result = Solution::largest_rectangle_area(num);
        println!("Result: {}", result);
    }
}
