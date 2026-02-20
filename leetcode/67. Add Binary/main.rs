impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        const padding_char: char = '0';
        let width = std::cmp::max(a.len(), b.len());

        let mut a = a;
        let mut b = b;

        // pad strings
        if a.len() < width {
            a = "0".repeat(width - a.len()) + &a;
        }
        if b.len() < width {
            b = "0".repeat(width - b.len()) + &b;
        }

        let mut a_chars = a.chars().rev();
        let mut b_chars = b.chars().rev();

        let mut result = "".to_string();
        let mut carry = 0;
        for i in (0..width) {
            let a_digit = a_chars.next().unwrap().to_digit(10).unwrap();
            let b_digit = b_chars.next().unwrap().to_digit(10).unwrap();
            let total = a_digit + b_digit + carry;

            carry = if total >= 2 { 1 } else { 0 };
            result.push(if total % 2 == 0 { '0' } else { '1' })
        }

        if(carry == 1) {
            result.push('1');
        }

        result.chars().rev().collect::<String>()
    }
}