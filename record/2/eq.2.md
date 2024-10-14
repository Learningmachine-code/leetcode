# main:
```rust
use leetcode::length_of_logest_substring;
fn main() {
    let a = String::from("hello word!");
    let b = length_of_logest_substring(a);
    println!("{b}");
}
```
# 解法1：
```rust
pub struct Solution {}
impl Solution {
    pub fn length_of_logest_substring(mut s: String) -> u32 {
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        let mut c = String::new();
        let mut t;
        for i in 0..s.len() {
            t = match s.pop() {
                Some(t) => t,
                none => continue
            };

            c.push(t);
            for j in c.chars() {
                if j == t {
                    a = 0;
                    c.clear();
                    break;
                }
            }
            if a >= b {
                b = a;
            }
        }
        b
    }
}
```
# 解法2
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
记录过程：
```rust
left=-1,last=0,ans=1
left=-1,last=1,ans=2
left=-1,last=2,ans=3
left=2,last=3,ans=3
left=2,last=4,ans=3
left=2,last=5,ans=3
left=4,last=6,ans=3
left=4,last=7,ans=3
left=4,last=8,ans=4
left=4,last=9,ans=5
5
```