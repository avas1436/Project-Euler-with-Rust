/*
198. House Robber

You are a professional robber planning to rob houses along a street. Each house
has a certain amount of money stashed, the only constraint stopping you from robbing
each of them is that adjacent houses have security systems connected and it will
automatically contact the police if two adjacent houses were broken into on the
same night.

Given an integer array nums representing the amount of money of each house, return
the maximum amount of money you can rob tonight without alerting the police.
 */

fn main() {
    let _ = Solution::rob(vec![1, 2, 3, 1]);
}

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut a = nums[0];
        if nums.len() == 1 {
            return a;
        }
        let mut b = std::cmp::max(a, nums[1]);

        for i in 2..nums.len() {
            let c = std::cmp::max(nums[i] + a, b);
            a = b;
            b = c;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_rob2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn test_rob3() {
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::rob(vec![]), 0);
    }

    #[test]
    fn test_single_house() {
        assert_eq!(Solution::rob(vec![5]), 5);
    }

    #[test]
    fn test_two_houses() {
        assert_eq!(Solution::rob(vec![2, 3]), 3);
    }

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 3);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::rob(vec![5, 5, 5, 5]), 10);
    }

    #[test]
    fn test_increasing() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5]), 9);
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(Solution::rob(vec![5, 4, 3, 2, 1]), 9);
    }

    #[test]
    fn test_with_zeros() {
        assert_eq!(Solution::rob(vec![0, 0, 0, 0]), 0);
    }

    #[test]
    fn test_zeros_between() {
        assert_eq!(Solution::rob(vec![2, 0, 2]), 4);
    }

    #[test]
    fn test_zero_edges() {
        assert_eq!(Solution::rob(vec![0, 5, 0, 5]), 10);
    }

    #[test]
    fn test_long_sequence() {
        assert_eq!(Solution::rob(vec![6, 7, 1, 30, 8, 2, 4]), 41);
    }

    #[test]
    fn test_alternating_large() {
        assert_eq!(Solution::rob(vec![100, 1, 100, 1, 100]), 300);
    }

    #[test]
    fn test_many_small_values() {
        let nums = vec![1; 20];
        assert_eq!(Solution::rob(nums), 10);
    }
}
