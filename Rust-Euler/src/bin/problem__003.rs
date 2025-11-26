use std::io;

fn main() {
    let number: i64 = loop {
        println!("Input a number to calculate biggest prime factor");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<i64>() {
            Ok(num) if num > 0 => break num,
            _ => println!("Please enter a valid positive number"),
        }
    };
    for i in (2..(number - 1)).rev() {
        if number % i == 0 {
            for j in 2..((i as f64).sqrt() as i64) {
                if i % j == 0 {
                } else {
                    println!("The biggest prime factor of {} is {}.", number, i);
                    break;
                }
                break;
            }
        }
    }
}
