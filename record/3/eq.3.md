# main
```rust
use leetcode::Solution;
fn main() {
    let a = String::from("helloword!");
    let b = Solution::length_of_longest_substring(a);
    println!("{b}");

}
```
# Solutions
## 参考答案
```rust
pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut last: [i32; 128] = [-1; 128];//一个用来存储下标的数组，在字符对应的索引
        let mut left = -1;//左指针，如何更新呢？left = max(left, last[v as usize]);这句话里面，last里面，如果一直不重复,返回值就一直是-1，如果重复之后，会返回上一个出现重复字符对应的在s里的下标，左指针更新并且指向他，
        let mut ans = 0;//记录出现过的最长的不重复字符串长度
        for (i, v) in s.chars().enumerate() {
            left = max(left, last[v as usize]);
            last[v as usize] = i as i32;
            ans = max(ans, (i as i32) - left);
            println!("left={left},last={},ans={ans}",last[v as usize]);
        }
        return ans;
    }
}
```

## 尝试用hashmap实现的我的答案
```rust
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
```

## 试试另一个hashmap函数
```rust

```