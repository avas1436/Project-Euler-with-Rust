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
        let mut a = nums[0];
        let mut b = nums[1];

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
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 3);
    }
}
