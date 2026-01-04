fn smallest_multiple(limit: i32) -> i64 {
    let mut step: i64 = limit as i64;

    loop {
        let mut status: bool = false;
        for i in 1..=limit {
            if step % i as i64 != 0 {
                step += 1;
                status = false;
                break;
            }
            status = true;
        }
        if status {
            break;
        }
    }
    step - 1
}

fn main() {
    let result: i64;
    let limit: i32;

    limit = 20;
    result = smallest_multiple(limit);
    eprintln!("the smallest multiple of {} is {}", limit, result);
}
