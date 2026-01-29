use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_problem_004(c: &mut Criterion) {
    c.bench_function("problem_004", |b| {
        b.iter(|| {
            let mut maximum = 0;
            for i in (100..1000).rev() {
                for j in (100..=i).rev() {
                    let product = i * j;
                    if product > maximum && is_palindrome(product) {
                        maximum = product;
                    }
                }
            }
            black_box(maximum); // جلوگیری از بهینه‌سازی بیش از حد توسط کامپایلر
        })
    });
}

fn is_palindrome(n: i32) -> bool {
    let str_num = n.to_string();
    let reverse: String = str_num.chars().rev().collect();
    reverse == str_num
}

criterion_group!(benches, bench_problem_004);
criterion_main!(benches);
