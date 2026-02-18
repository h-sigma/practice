impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        let mut sum = 0;
        let len = salary.len();

        for item in salary {
            max = std::cmp::max(item, max);
            min = std::cmp::min(item, min);
            sum += item;
        }

        sum -= max;
        sum -= min;
        (sum as f64) / ((len - 2) as f64)
    }
}