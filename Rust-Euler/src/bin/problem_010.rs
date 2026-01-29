use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let start_generate = Instant::now();

    let primes = generate_primes_up_to(2_000_000);

    let start_sum = Instant::now();

    let sum_primes = sum_primes_below(primes);

    let end_generate = start_sum.duration_since(start_generate);
    let end_sum = start_sum.elapsed();

    println!("generating prime list in : {:?}", end_generate);
    println!("sum primes in : {:?}", end_sum);
    println!("total time : {:?}", (end_generate + end_sum));
    println!("sum of all primes below 2-milion is : {}", sum_primes);
}

fn sum_primes_below(primes: HashSet<usize>) -> usize {
    let mut sum: usize = 0;

    for prime in primes {
        sum += prime;
    }
    sum
}

fn generate_primes_up_to(limit: usize) -> HashSet<usize> {
    let mut primes_set: HashSet<usize> = HashSet::new();
    let mut candidate: usize = 3;
    primes_set.insert(2);

    'prime: loop {
        if candidate > limit {
            break;
        }
        if candidate % 2 == 0 {
            candidate += 1;
            continue;
        }
        let mut divisor: usize = 3;
        while divisor * divisor <= candidate {
            if candidate % divisor == 0 {
                candidate += 1;
                continue 'prime;
            }
            divisor += 2;
        }
        primes_set.insert(candidate);
        candidate += 1;
    }
    primes_set
}

#[cfg(test)]
mod tests {
    use crate::generate_primes_up_to;
    use crate::sum_primes_below;

    #[test]
    fn test_generate_primes_up_to_10() {
        let primes = generate_primes_up_to(10);
        assert!(primes.contains(&2), "Dosnt have 2");
        assert!(primes.contains(&3), "Dosnt have 3");
        assert!(primes.contains(&5), "Dosnt have 5");
        assert!(primes.contains(&7), "Dosnt have 7");
        assert_eq!(primes.len(), 4, "worng length");
    }

    #[test]
    fn sum_primes_below_10() {
        let primes = generate_primes_up_to(20);
        let result = sum_primes_below(primes);
        assert_eq!(result, 77, "wrong sum for primes below 20")
    }
}
