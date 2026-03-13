impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;

        // sort intervals by start
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut overlaps: Vec<Vec<i32>> = vec![];
        let mut overlap = intervals[0].to_owned(); // could use an option here instead, but honestly it's messier

        // loop over sorted intervals and merge them if they overlap, otherwise add the current overlap to the result and start a new one
        for i in 0..intervals.len() {
            if intervals[i][0] <= overlap[1] {
                overlap[1] = std::cmp::max(intervals[i][1], overlap[1]);
            } else {
                overlaps.push(overlap);
                overlap = intervals[i].to_owned();
            }
        }

        overlaps.push(overlap);

        overlaps
    }
}