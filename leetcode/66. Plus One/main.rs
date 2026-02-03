use std::cmp::Ordering;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut carry = 1;
        for i in (0..digits.len()).rev() {
            let mut new_digit = digits[i] + carry;
            carry = 0;
            match new_digit.cmp(&10) {
                Ordering::Less => {
                    carry = 0;
                }
                _ => {
                    carry = 1;
                    new_digit -= 10;
                }
            }
            result.push(new_digit);
        }
        if carry > 0 {
            result.push(1);
        }
        result.into_iter().rev().collect()
    }
}