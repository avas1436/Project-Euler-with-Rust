// monotonic_stack
// Q3. Largest Rectangle in Histogram
//
// Given an array of integers heights representing the histogram's bar height where the
//  width of each bar is 1, return the area of the largest rectangle in the histogram.

// use std::cmp::min;
struct Solution;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut maximum: i32 = 0;
        let mut stack: Vec<usize> = Vec::new();

        heights.push(0);

        for i in 0..heights.len() {
            let mut min_height: i32 = -1;
            while !stack.is_empty() && heights[i] < heights[*stack.last().unwrap()] {
                if min_height == -1 {
                    min_height = heights[stack.pop().unwrap()]
                } else if min_height > heights[stack.pop().unwrap()] {
                    min_height = heights[stack.pop().unwrap()]
                }
                let width: i32 = if stack.is_empty() {
                    i as i32
                } else {
                    (i - stack.last().unwrap() - 1) as i32
                };
                maximum = maximum.max(min_height * width);
            }
            stack.push(i);
        }

        maximum
    }
}

fn main() {
    let num1: Vec<i32> = vec![2, 1, 5, 6, 2, 3];
    let num2: Vec<i32> = vec![1, 1];
    let num3: Vec<i32> = vec![2, 4];
    let num4: Vec<i32> = vec![2, 1, 2];
    for num in [num1, num2, num3, num4] {
        let result = Solution::largest_rectangle_area(num);
        println!("Result: {}", result);
    }
}
