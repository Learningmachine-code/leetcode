# main

## My solution 1

---

```rust
let mut record = String::new();
        let v = &s.chars().collect::<Vec<char>>();
        let mut length: usize = 0;
        let mut left: usize = 0;
        let mut middle: usize = s.len() / 2;
        let mut right: usize = s.len() - 1;
    }
    fn find(
        v: &Vec<char>,
        length: &mut u32,
        middle: &mut usize,
        left: &mut usize,
        right: &mut usize,
    ) {
        if v.get(*left).is_empty
        if v[*middle] == v[*left] && *middle - *left == 1 {
            *length = 2;
            *left -= 1;
        }
        if v[*middle] == v[*right] && *right - *middle == 1 {
            *length = 2;
            *right += 1;
        }
        if v[*left] == v[*right] {
            *length += 2;
            *left -= 1;
            *right += 1;
            return Self::find();
        }
        if v[*left] !=v[*right]{
            middle =right;
            right=middle+1;
            left=middle-1;
            return Self::find();
        }

```

## other solutions

---

```rust
pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let s: Vec<char> = s.chars().collect();

        let mut dp = vec![vec![true; len]; len];
        let mut res = (0, 0);

        for k in 1..len {
            for i in 0..(len - k) {
                if k == 1 {
                    dp[i][i + k] = s[i] == s[i + 1];
                } else {
                    dp[i][i + k] = (s[i] == s[i + k]) && dp[i + 1][i + k - 1];
                }
                if dp[i][i + k] {
                    res = (i, i + k);
                }
            }
        }
        s[res.0..=res.1].into_iter().collect::<String>()
}

```
