use std::collections::BinaryHeap;

/*
Q2. Find K Pairs with Smallest Sums

You are given two integer arrays nums1 and nums2 sorted in non-decreasing order and an
integer k.

Define a pair (u, v) which consists of one element from the first array and one element
from the second array.

Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.
*/
fn main() {
    todo!("Problem solved here");
}

struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut num1_bh = BinaryHeap::from(nums1.clone());
        let mut num2_bh = BinaryHeap::from(nums1.clone());
        let limit: usize;

        if nums1.len() <= nums2.len() {
            limit = nums1.len() - k as usize;
        } else {
            limit = nums2.len() - k as usize;
        }

        for _ in 0..limit {
            num1_bh.pop();
            num2_bh.pop();
        }
        for _ in 0..limit {
            ans.push(vec![num1_bh.pop().unwrap(), num2_bh.pop().unwrap()]);
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
