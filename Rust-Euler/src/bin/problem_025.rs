// problem number 25 of euler project

use num_traits::{One, Zero};
use numbigint::bigUint;

fn answare(leng: u128) -> u128 {
    let mut a: BigUnit = One::one();
    let mut b: BigUnit = One::one();
    let mut step: u128 = 2;
    loop {
        (b, a) = (a + b, b);
        step += 1;
        println!("step is : {step}, num is : {b}");
        let len = number_of_digits(b);
        if len == leng {
            return step;
        }
    }
}

fn number_of_digits(number: u128) -> u128 {
    let mut len = 0;
    let mut num = number.clone();
    while num != 0 {
        num /= 10;
        len += 1;
    }
    len
}

fn main() {
    let ans: u128 = answare(1000);
    println! {"the answare is {ans}"};
}
