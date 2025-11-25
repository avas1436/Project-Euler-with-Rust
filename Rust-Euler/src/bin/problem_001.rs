fn main() {
    let mut sum: i32 = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i
        };
    }
    println! {"sum of 3 and 5 divisables is {}", sum};
}
