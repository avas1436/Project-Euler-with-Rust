// problem number 25 of euler project

fn answare(leng: u128) -> u128 {
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    let mut step: u128 = 2;
    loop {
        (b, a) = (a + b, b);
        step += 1;
        let len = digit_number(b);
        if len == leng {
            return step;
        }
    }
}

fn digit_number(number: u128) -> u128 {
    let mut len = 0;
    let mut num = number.clone();
    while num != 0 {
        num /= 10;
        len += 1;
    }
    len
}

fn main() {
    let ans: u128 = answare(100);
    println! {"the answare is {ans}"};
}
