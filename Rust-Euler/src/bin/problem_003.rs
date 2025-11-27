use std::io;

fn main() {
    let num: u64 = take_number();
    for i in (((num as f64).sqrt() as u64)..num).rev() {
        if num % i == 0 && is_prime(i) {
            println!("The biggest prime factor of {} is {}", num, i);
            break;
        };
    }
}

// let mut n: bool = false;

// for i in (1..30).rev() {
//     n = is_prime(i);
//     if n {
//         println!("number {} is prime", i);
//     }
// }

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    };
    if n == 2 {
        return true;
    };
    if n % 2 == 0 {
        return false;
    };

    let mut i: u64 = 3;

    while i * i < n {
        if n % i == 0 {
            return false;
        };
        i += 2;
    }
    true
}

fn take_number() -> u64 {
    loop {
        println!("Input a number to calculate biggest prime factor");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<u64>() {
            Ok(num) if num > 0 => return num,
            _ => println!("Please enter a valid positive number"),
        }
    }
}
