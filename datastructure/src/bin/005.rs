use std::collections::VecDeque;

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
    let answare = Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]);
    println!("{}", answare);
}

struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut stud_queue: VecDeque<i32> = VecDeque::from(students);
        let mut failed_attempt = 0;
        let mut sand_step = 0;

        while !stud_queue.is_empty() && sandwiches.len() > failed_attempt {
            let student = stud_queue.pop_front().unwrap();
            if sandwiches[sand_step] == student {
                failed_attempt = 0;
                sand_step += 1;
                continue;
            } else {
                stud_queue.push_back(student);
                failed_attempt += 1;
            }
        }
        stud_queue.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_answare() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
    }

    #[test]
    fn test_three_students_without_lunch() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }

    #[test]
    fn test_all_students_without_lunch() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0]),
            6
        );
    }

    #[test]
    fn test_all_students_with_lunch() {
        assert_eq!(
            Solution::count_students(vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0]),
            0
        );
    }

    #[test]
    fn all_empty() {
        assert_eq!(Solution::count_students(vec![], vec![]), 0);
    }
}
