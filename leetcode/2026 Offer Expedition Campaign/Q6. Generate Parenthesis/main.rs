impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut current_string = String::with_capacity(2 * n as usize);

        Solution::backtrack(&mut result, &mut current_string, 0, 0, n);

        result
    }

    fn backtrack(result: &mut Vec<String>, current_string: &mut String, open: i32, closed: i32, max: i32) {
        if current_string.len() == (max * 2) as usize {
            result.push(current_string.clone());
            return;
        }

        if open < max {
            current_string.push('('); // new state
            Solution::backtrack(result, current_string, open + 1, closed, max); // explore
            current_string.pop(); // backtrack
        }

        if closed < max && closed < open {
            current_string.push(')'); // new state
            Solution::backtrack(result, current_string, open, closed + 1, max); // explore
            current_string.pop(); // backtrack
        }
    }
}