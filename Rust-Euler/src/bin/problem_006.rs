fn main() {
    struct Point(i32, i32);

    let p = Point(10, 20);
    println!("x={}, y={}", p.0, p.1);
}
