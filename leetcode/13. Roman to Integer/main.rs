impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // collect by chars because we want random access to the characters
        let chars: Vec<_> = s.chars().collect();
        let mut sum = 0;
        let mut i = 0;

        while i < chars.len() {
            let cur = chars[i];
            i += 1;
            if i < chars.len() {
                // check for pairs first, if not at end of string
                let next = chars[i];
                let x = match (cur, next) {
                    ('I','V') => 4,
                    ('I','X') => 9,
                    ('X','L') => 40,
                    ('X','C') => 90,
                    ('C','D') => 400,
                    ('C','M') => 900,
                    _ => 0,
                };
                if x > 0 {
                    // pair found, add total value to sum and skip second char of pair from being read in next iteration
                    sum += x;
                    i += 1;
                    continue;
                }
            }

            sum += match cur {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("Ill-formatted roman numeral.")
            };
        }
        sum
    }
}