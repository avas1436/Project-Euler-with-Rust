/*
746. Min Cost Climbing Stairs

You are given an integer array cost where cost[i] is the cost of ith step on a
staircase. Once you pay the cost, you can either climb one or two steps.

You can either start from the step with index 0, or the step with index 1.

Return the minimum cost to reach the top of the floor.
 */

fn main() {
    let _ = Solution::min_cost_climbing_stairs(vec![10, 15, 20]);
}

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut prev2 = cost[0];
        let mut prev1 = cost[1];

        for i in 2..cost.len() {
            let current = cost[i] + std::cmp::min(prev1, prev2);
            prev2 = prev1;
            prev1 = current;
        }

        std::cmp::min(prev1, prev2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let cost = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
    }

    #[test]
    fn example_2() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
    }

    #[test]
    fn edge_small() {
        let cost = vec![5, 10];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 5);
    }
}
