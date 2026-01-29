// Sum Square Difference

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut sum_squares: i64 = 0;
    let mut squares_sum: i64 = 0;
    for i in 1..=100 {
        sum_squares += i * i;
        squares_sum += i;
    }
    squares_sum *= squares_sum;
    let ans = squares_sum - sum_squares;

    let duration = start.elapsed();
    println!(
        "difference between the sum of the squares and the square of the sum : {}",
        ans
    );
    println!("Solve in : {:?} seconds", duration);
}
