impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut n = n as i64; // Use i64 to handle the edge case: i32::MIN
        
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        let mut result = 1.0;
        let mut current_product = x;

        // when n is odd, x^n == x * x ^ (n - 1)
        // when n is even, x^n == (x * x) ^ (n / 2)

        while n > 0 {
            if n % 2 == 1 {
                // If n is odd, multiply the result by the current power of x
                result *= current_product;
                // n -= 1; // we don't need to do this because division is truncating in rust
            }
            // Square the base and halve the exponent
            current_product *= current_product;
            n /= 2;
        }

        result
    }
}