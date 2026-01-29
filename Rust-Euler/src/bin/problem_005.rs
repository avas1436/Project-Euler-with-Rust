use std::time::Instant;

fn smallest_multiple(limit: i32) -> i64 {
    let mut step: i64 = limit as i64;

    loop {
        let mut status: bool = false;
        for i in 1..=limit {
            if step % i as i64 != 0 {
                status = false;
                break;
            }
            status = true;
        }
        if status {
            break;
        }
        step += 1;
    }
    step - 1
}

fn main() {
    let start = Instant::now();
    let result: i64;
    let limit: i32;

    limit = 20;
    result = smallest_multiple(limit);
    let duration = start.elapsed();
    eprintln!("the smallest multiple of {} is {}", limit, result);
    eprintln!("elapsed time: {:?}", duration);
}
