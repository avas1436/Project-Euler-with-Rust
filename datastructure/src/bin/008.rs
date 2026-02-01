/*
Q1. Last Stone Weight

You are given an array of integers stones where stones[i] is the weight of the ith stone.

We are playing a game with the stones. On each turn, we choose the heaviest two stones
and smash them together. Suppose the heaviest two stones have weights x and y with
x <= y. The result of this smash is:

    If x == y, both stones are destroyed, and
    If x != y, the stone of weight x is destroyed, and the stone of weight y has new
    weight y - x.

At the end of the game, there is at most one stone left.

Return the weight of the last remaining stone. If there are no stones left, return 0.
*/

use std::collections::BinaryHeap;

fn main() {
    let ans = Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]);
    println!("answare is : {}", ans);
}

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);
        let mut y: i32;
        let mut x: i32;

        while heap.len() > 1 {
            y = heap.pop().unwrap();
            x = heap.pop().unwrap();

            if x == y {
                continue;
            } else {
                heap.push(y - x);
            }
        }
        if heap.len() == 0 {
            0
        } else {
            heap.pop().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_stone_weight() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn just_one_stone() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }
}
