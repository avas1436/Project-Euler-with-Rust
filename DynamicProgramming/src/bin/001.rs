/*
70. Climbing Stairs

You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct
ways can you climb to the top?
*/

fn main() {
    todo!("problem solve here");
}

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut step: usize = n as usize;
        let mut a: i32 = 1;
        if n == 1 {
            return a;
        }
        let mut b: i32 = 2;
        if n == 2 {
            return b;
        }

        while step > 2 {
            (b, a) = (a + b, b);
            step -= 1;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::climb_stairs(5), 8);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::climb_stairs(6), 13);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::climb_stairs(7), 21);
    }

    #[test]
    fn test_8() {
        assert_eq!(Solution::climb_stairs(8), 34);
    }

    #[test]
    fn test_9() {
        assert_eq!(Solution::climb_stairs(9), 55);
    }
}
