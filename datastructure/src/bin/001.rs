struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let maximum: i32 = 0;

        maximum
    }
}

fn main() {
    let num1: Vec<i32> = vec![2, 1, 5, 6, 2, 3];
    let num2: Vec<i32> = vec![2, 1, 5, 6, 2, 3];
    let num3: Vec<i32> = vec![2, 1, 5, 6, 2, 3];
    for num in [num1, num2, num3] {
        let result = Solution::largest_rectangle_area(num);
        println!("Result: {}", result);
    }
}
