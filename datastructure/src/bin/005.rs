/*
Q1. Number of Students Unable to Eat Lunch


The school cafeteria offers circular and square sandwiches at lunch break, referred to
by numbers 0 and 1 respectively. All students stand in a queue. Each student either
prefers square or circular sandwiches.

The number of sandwiches in the cafeteria is equal to the number of students.
The sandwiches are placed in a stack. At each step:

    If the student at the front of the queue prefers the sandwich on the top of the
    stack, they will take it and leave the queue.
    Otherwise, they will leave it and go to the queue's end.

This continues until none of the queue students want to take the top sandwich and
are thus unable to eat.

You are given two integer arrays students and sandwiches where sandwiches[i] is
the type of the i th sandwich in the stack (i = 0 is the top of the stack)
and students[j] is the preference of the j th student in the initial
queue (j = 0 is the front of the queue). Return the number of students
that are unable to eat.
*/
fn main() {
    todo!("run Queue here!");
}

struct Solution;

impl Solution {
    pub fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut sand: i32 = 0;
        let mut stud: i32 = 0;
        let mut step = 0;
        while step < sandwiches.len() {
            if sand == 2 {
                sand = sandwiches.pop().unwrap();
            }
            stud = students.pop().unwrap();
            if sand == stud {
                sand = 2;
            } else {
                students.insert(0, stud);
            }
            step += 1;
        }
        students.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_students() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
