/* Q2. Time Needed to Buy Tickets

There are n people in a line queuing to buy tickets, where the 0th person is at the front
of the line and the (n - 1)th person is at the back of the line.

You are given a 0-indexed integer array tickets of length n where the number of tickets
that the ith person would like to buy is tickets[i].

Each person takes exactly 1 second to buy a ticket. A person can only buy 1 ticket at a
time and has to go back to the end of the line (which happens instantaneously) in order
to buy more tickets. If a person does not have any tickets left to buy, the person will
leave the line.

Return the time taken for the person initially at position k (0-indexed) to finish buying
tickets.
*/

fn main() {
    todo!("run solution here!")
}

struct Solution;
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {

    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_time_required_to_buy_6() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
    }

    #[test]
    fn test_time_required_to_buy_8() {
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
