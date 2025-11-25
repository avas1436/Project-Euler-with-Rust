fn main() {
    let mut sum: i64 = 2;
    let mut current: i64 = 2;
    let mut previous: i64 = 1;
    let mut middle: i64 = 0;
    //let mut step: i32 = 2;
    while current < 4_000_000 {
        //step += 1;
        middle = current;
        current += previous;
        previous = middle;
        // println!(
        //     "current is : {} and middle is {} on step : {}",
        //     current, middle, step
        // );
        if current % 2 == 0 {
            sum += current;
        }
    }
    print!("The finall answare is : {}", sum)
}
