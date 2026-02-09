/*
I have not used the standard library stack implementation because I wanted to practice writing my own.
It's cleaner than using a vector and doing ugly index arithmetic.
And this is more in the spirit of the problem.
*/
struct Stack {
    v: Vec<i32>
}      

impl Stack {
    pub fn new() -> Stack {
        Stack {
            v: Vec::new(),
        }
    }

    pub fn push(&mut self, item: i32) {
        self.v.push(item);
    }

    pub fn peek(&self, at: usize) -> &i32 {
        &self.v[self.v.len() - 1 - at]
    }

    pub fn pop(&mut self) {
        self.v.pop();
    }

    pub fn total(self) -> i32 {
        self.v.into_iter().reduce(|acc, x| acc + x).unwrap_or_default()
    }
}

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut record = Stack::new();
        for op in operations {
            match op.as_str() {
                "+" => record.push(record.peek(0) + record.peek(1)),
                "D" => record.push(record.peek(0) * 2),
                "C" => record.pop(),
                s => record.push(s.parse::<i32>().unwrap())
            }
        }
        record.total()
    }
}