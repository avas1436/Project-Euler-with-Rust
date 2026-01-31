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
    // Your MyQueue object will be instantiated and called as such:
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);
    obj.push(3);
    let ret_2: i32 = obj.pop();
    println!("FIFO is : {ret_2}");
    let ret_3: i32 = obj.peek();
    println!("FIFO is : {ret_3}");
    let ret_4: bool = obj.empty();
    println!("is empty : {ret_4}");
}

struct MyQueue {
    fifo: Vec<i32>,
    lifo: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            lifo: Vec::new(),
            fifo: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.lifo.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.fifo.is_empty() {
            while self.lifo.len() > 1 {
                self.fifo.push(self.lifo.pop().unwrap());
            }
            return self.lifo.pop().unwrap();
        } else {
            return self.fifo.pop().unwrap();
        }
    }

    fn peek(&mut self) -> i32 {
        if self.fifo.is_empty() {
            while !self.lifo.is_empty() {
                self.fifo.push(self.lifo.pop().unwrap());
            }
            return *self.fifo.last().unwrap();
        } else {
            return *self.fifo.last().unwrap();
        }
    }

    fn empty(&self) -> bool {
        return self.fifo.is_empty() && self.lifo.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MyQueueWorld {
        command: Vec<String>,
        input: Vec<Vec<i32>>,
    }

    impl MyQueueWorld {
        fn new(command: Vec<String>, input: Vec<Vec<i32>>) -> Self {
            Self { command, input }
        }

        fn run(&self) -> Vec<Option<i32>> {
            let mut ans: MyQueue = MyQueue::new();
            let mut result: Vec<Option<i32>> = Vec::new();

            for (cmd, inp) in self.command.iter().zip(self.input.iter()) {
                match cmd.as_str() {
                    "MyQueue" => {
                        result.push(None);
                    }
                    "push" => {
                        ans.push(inp[0]);
                        result.push(None);
                    }
                    "peek" => result.push(Some(ans.peek())),
                    "pop" => result.push(Some(ans.pop())),
                    "empty" => result.push(Some(ans.empty() as i32)),
                    _ => println!("nothing!"),
                }
            }
            result
        }
    }

    #[test]
    fn leet_code_test() {
        let result = MyQueueWorld::new(
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
        let final_result = result.run();
        assert_eq!(
            final_result,
            vec![None, None, None, Some(1), Some(1), Some(0)]
        );
    }
}
