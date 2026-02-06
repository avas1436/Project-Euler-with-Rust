/*
Q3. Construct Target Array With Multiple Sums

You are given an array target of n integers. From a starting array arr consisting of
n 1's, you may perform the following procedure :

    let x be the sum of all elements currently in your array.
    choose index i, such that 0 <= i < n and set the value of arr at index i to x.
    You may repeat this procedure as many times as needed.

Return true if it is possible to construct the target array from arr, otherwise, return
false.
 */
fn main() {
    let target = vec![9, 3, 5];
    println!("{}", Solution::is_possible(target));
}

struct Solution;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut heap = std::collections::BinaryHeap::new();
        let mut sum = 0;

        for &num in &target {
            heap.push(num);
            sum += num;
        }

        while let Some(max) = heap.pop() {
            if max == 1 {
                return true;
            }

            let rest = sum - max;
            if rest == 1 {
                return true;
            }

            let next = max % rest;
            if next == 0 || next == max {
                return false;
            }

            heap.push(next);
            sum = rest + next;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_possible() {
        assert_eq!(Solution::is_possible(vec![9, 3, 5]), true);
    }

    #[test]
    fn test_is_possible_2() {
        assert_eq!(Solution::is_possible(vec![1, 1, 1, 2]), false);
    }
}
