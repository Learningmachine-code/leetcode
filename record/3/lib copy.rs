pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut recorder = HashMap::new();
        let mut left: i32 = -1;
        let mut ans: i32 = 0;
        let mut last: i32;
        for (i, v) in s.chars().enumerate() {
            last = match recorder.get(&v) {
                Some(&v) => v,
                None => -1,
            };
            left = max(left, last);
            recorder.insert(v, i as i32);
            ans = max(ans, (i as i32) - left);
        }
        ans
    }
}
