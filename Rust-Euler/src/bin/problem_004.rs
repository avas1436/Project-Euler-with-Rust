fn main() {
    let mut maximum = 0;
    for i in (100..1000).rev() {
        for j in (i..1000).rev() {
            let product = i * j;
            if product < maximum {
                break;
            }
            if is_palindrome(product) {
                maximum = product;
            }
        }
    }
    println!("The biggest palindrome is : {maximum}.")
}
fn is_palindrome(n: i32) -> bool {
    let str_num = n.to_string();
    let reverse = str_num.chars().rev().collect::<String>();
    reverse == str_num
}
