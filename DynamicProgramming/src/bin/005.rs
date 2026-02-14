/*
 *
62. Unique Paths

There is a robot on an m x n grid. The robot is initially located at the top-left
corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner
(i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any
point in time.

Given the two integers m and n, return the number of possible unique paths that the
robot can take to reach the bottom-right corner.

The test cases are generated so that the answer will be less than or equal to 2 * 109.
 */

fn main() {
    let x = Solution::unique_paths(3, 7);
    println!("The answare for 3 * 7 is {x}");
}

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let x = m as usize;
        let y = n as usize;
        let mut robot_memory: Vec<Vec<i32>> = vec![vec![0; y]; x];

        for n in 0..x {
            for m in 0..y {
                println!("n:{}, m:{}", n, m);
                if n == 0 || m == 0 {
                    robot_memory[n][m] = 1;
                } else {
                    robot_memory[n][m] = robot_memory[n - 1][m] + robot_memory[n][m - 1];
                }
            }
        }

        robot_memory[x - 1][y - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn test_unique_paths_2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn test_unique_paths_3() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }

    #[test]
    fn test_unique_paths_4() {
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }

    #[test]
    fn test_unique_paths_5() {
        assert_eq!(Solution::unique_paths(2, 2), 2);
    }

    #[test]
    fn test_unique_paths_6() {
        assert_eq!(Solution::unique_paths(1, 1), 1);
    }
}
