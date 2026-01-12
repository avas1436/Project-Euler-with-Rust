// problem number 25 of euler project

fn answare(leng: u128) -> u128 {
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    let mut step: u128 = 2;
    loop {
        (b, a) = (a + b, b);
        step += 1;
        if b.to_string().len() == leng as usize {
            return step;
        }
    }
}

fn main() {
    let ans: u128 = answare(50);
    println! {"the answare is {ans}"};
}
