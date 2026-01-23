use std::collections::HashSet;

fn main() {
    println!("hello, world");
}

// fn sum_primes_below(primes: HashSet<usize>) -> usize {
//     println!("Ok!");
// }

fn generate_primes_up_to(limit: usize) -> HashSet<usize> {
    let mut primes_set: HashSet<usize> = HashSet::new();
    let mut candidate: usize = 1;

    'prime: loop {
        if candidate == 1 || candidate == 2 {
            continue;
        } else if candidate % 2 == 0 {
            continue;
        }
        let mut divisor: usize = 3;
        while divisor * divisor < candidate {
            if candidate % divisor == 0 {
                break 'prime;
            }
            divisor += 2;
        }
        primes_set.insert(candidate);
        candidate += 1;
        if candidate > limit {
            break;
        }
    }
    primes_set
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::generate_primes_up_to;
    // use crate::sum_primes_below;

    #[test]
    fn test_generate_primes_up_to_10() {
        let primes = generate_primes_up_to(10);
        assert!(primes.contains(&2));
        assert!(primes.contains(&3));
        assert!(primes.contains(&5));
        assert!(primes.contains(&7));
        assert_eq!(primes.len(), 4);
    }

    // #[test]
    // fn sum_primes_below_10() {
    //     let result = sum_primes_below(10);
    //     assert_eq!(result, 12, "wrong sum for primes below 10")
    // }
}
