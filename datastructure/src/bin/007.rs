/* Q3. Implement Queue using Stacks

Implement a first in first out (FIFO) queue using only two stacks. The implemented
queue should support all the functions of a normal queue (push, peek, pop, and empty).

Implement the MyQueue class:

    void push(int x) Pushes element x to the back of the queue.
    int pop() Removes the element from the front of the queue and returns it.
    int peek() Returns the element at the front of the queue.
    boolean empty() Returns true if the queue is empty, false otherwise.

Notes:

    You must use only standard operations of a stack, which means only push to top,
    peek/pop from top, size, and is empty operations are valid.
    Depending on your language, the stack may not be supported natively. You may
    simulate a stack using a list or deque (double-ended queue) as long as you use only
    a stack's standard operations.
*/

fn main() {
    todo!("answare run here!");
}

struct MyQueue {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {}

    fn push(&self, x: i32) {}

    fn pop(&self) -> i32 {}

    fn peek(&self) -> i32 {}

    fn empty(&self) -> bool {}
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    struct MyQueueWorld {
        command: Vec<String>,
        input: Vec<Vec<i32>>,
    }

    impl MyQueueWorld {
        fn new(command: Vec<String>, input: Vec<Vec<i32>>) -> Self {
            Self { command, input }
        }

        fn run(&self) -> Vec<Option<i32>> {
            let mut ans: Option<MyQueue> = None;
            let mut result: Vec<Option<i32>> = Vec::new();

            for (cmd, inp) in self.command.iter().zip(self.input.iter()) {
                match cmd.as_str() {
                    "MyQueue" => {
                        ans = Some(MyQueue::new());
                        result.push(None);
                    }
                    "push" => {
                        ans.as_mut().unwrap().push(inp[0]);
                        result.push(None);
                    }
                    "peek" => result.push(Some(ans.peek())),
                    "pop" => result.push(Some(ans.pop())),
                    "empty" => result.push(Some(ans.empty() as i32)),
                }
            }
            result
        }
    }

    #[test]
    fn leet_code_test() {
        let mut result = MyQueueWorld::new(
            vec![
                "MyQueue".to_string(),
                "push".to_string(),
                "push".to_string(),
                "peek".to_string(),
                "pop".to_string(),
                "empty".to_string(),
            ],
            vec![vec![], vec![1], vec![2], vec![], vec![], vec![]],
        );
        result.run();
        assert_eq!(result, vec![None, None, None, Some(1), Some(1), None]);
    }
}
