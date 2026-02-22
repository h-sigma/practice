impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let digits1 = Solution::to_reversed_indexed_intvec(num1);
        let digits2 = Solution::to_reversed_indexed_intvec(num2);

        let mut raw_sums = vec![0; digits1.len() + digits2.len() - 1];
        for (i, vi) in digits1 {
            for (j, vj) in &digits2 {
                raw_sums[i + j] += vi * vj;
            }
        }
        
        let mut result = vec![];
        let final_carry = raw_sums.into_iter().fold(0, |s, carry| {
            let sum = s + carry;
            result.push(sum % 10);
            sum / 10
        });
        
        if final_carry > 0 {
            result.push(final_carry);
        }

        let ans: String = result
            .into_iter()
            .rev()                   
            .skip_while(|&d| d == 0) 
            .map(|d| d.to_string())  
            .collect();

        if ans.is_empty() {
            "0".to_string()
        } else {
            ans
        }
    }

    fn to_reversed_indexed_intvec(nums: String) -> Vec<(usize, u32)> {
        nums.chars().map(|c| c.to_digit(10).unwrap()).rev().enumerate().collect()
    }
}