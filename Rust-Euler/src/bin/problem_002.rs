fn main() {
    let mut sum: i64 = 2;
    let mut current: i64 = 2;
    let mut previous: i64 = 1;
    let mut temp: i64 = 0;
    //let mut step: i32 = 2;
    while current < 4_000_000 {
        //step += 1;
        temp = current;
        current += previous;
        previous = temp;
        // println!(
        //     "current is : {} and temp is {} on step : {}",
        //     current, temp, step
        // );
        if current % 2 == 0 {
            sum += current;
        }
    }
    print!("The finall answare is : {}", sum)
}
