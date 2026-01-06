// monotonic_stack
// Q3. Largest Rectangle in Histogram
//
// Given an array of integers heights representing the histogram's bar height where the
//  width of each bar is 1, return the area of the largest rectangle in the histogram.

// use std::cmp::min;
struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut area: i32 = 0;
        let mut maximum: i32 = 0;
        let mut stack: Vec<usize> = Vec::new();
        let mut cur_index: usize = 0;
        let mut cur_heigh: i32 = 0;

        for i in 0..heights.len() {
            let mut step: i32 = 0;
            let mut min_height: i32 = -1;
            while !stack.is_empty() {
                cur_index = stack.pop().unwrap();
                cur_heigh = heights[cur_index];
                if min_height == -1 {
                    min_height = cur_heigh;
                } else if min_height > cur_heigh {
                    min_height = cur_heigh;
                }
                step += 1;
                area = min_height * step;
                if area > maximum {
                    maximum = area;
                }
            }
            stack.push(i);
        }

        maximum
    }
}

fn main() {
    let num1: Vec<i32> = vec![2, 1, 5, 6, 2, 3];
    let num2: Vec<i32> = vec![2, 1, 5, 6, 2, 3];
    let num3: Vec<i32> = vec![2, 4];
    for num in [num1, num2, num3] {
        let result = Solution::largest_rectangle_area(num);
        println!("Result: {}", result);
    }
}
