use std::{cmp::Reverse, collections::BinaryHeap};

/*
Q2. Find K Pairs with Smallest Sums

You are given two integer arrays nums1 and nums2 sorted in non-decreasing order and an
integer k.

Define a pair (u, v) which consists of one element from the first array and one element
from the second array.

Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.
*/
fn main() {
    let nums1 = vec![1, 7, 11];
    let nums2 = vec![2, 4, 6];
    let k = 3;
    let ans = Solution::k_smallest_pairs(nums1, nums2, k);
    println!("{:?}", ans);
}

struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        let limit: usize = k as usize;

        for i in 0..nums1.len().min(limit) {
            heap.push(Reverse(((nums1[i] + nums2[0]), i, 0)));
        }

        while ans.len() < limit && !heap.is_empty() {
            let Reverse((_, i, j)) = heap.pop().unwrap();
            ans.push(vec![nums1[i], nums2[j]]);
            if j + 1 < nums2.len() {
                heap.push(Reverse(((nums1[i] + nums2[j + 1]), i, j + 1)));
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_smallest_pairs() {
        let nums1 = vec![1, 7, 11];
        let nums2 = vec![2, 4, 6];
        let k = 3;
        let expected = vec![vec![1, 2], vec![1, 4], vec![1, 6]];
        assert_eq!(Solution::k_smallest_pairs(nums1, nums2, k), expected);
    }

    #[test]
    fn test_k_smallest_pairs_2() {
        let nums1 = vec![1, 1, 2];
        let nums2 = vec![1, 2, 3];
        let k = 2;
        let expected = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::k_smallest_pairs(nums1, nums2, k), expected);
    }
}
