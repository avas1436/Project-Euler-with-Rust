use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn smallest_multiple(limit: i32) -> i64 {
    let mut step: i64 = limit as i64;

    loop {
        let mut divisible = true;
        for i in 1..=limit {
            if step % i as i64 != 0 {
                divisible = false;
                break;
            }
        }
        if divisible {
            return step;
        }
        step += 1;
    }
}

fn bench(c: &mut Criterion) {
    c.bench_function("problem 5", |b| {
        b.iter(|| {
            let limit = 20;
            let result = smallest_multiple(limit);
            black_box(result);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
