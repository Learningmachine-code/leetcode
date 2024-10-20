# main

```rust
use testcode::Solution;
fn main() {
    let input = String::from("asdfghqwerrewqadsvedbesvgb");
    let result = Solution::logest_palindrome(input);
    println!("The result is {result}");
}
```

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

### `S1`

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

### `S1` annotation

```rust
pub struct Solution {}
impl Solution {
    pub fn logest_palindrome(s: String) -> String {
        // The `String` isn`t used by index in rust.
        // So convert the string to `Vec<char>`.
        let s: Vec<char> = s.chars().collect();
        let n = s.len();

        // Two-dimentsional-list
        // Dynamic programming;
        // Let dp[i][j]=(s[i]==s[j] && dp[i+1][j-1];
        // The `dp[i][j]` means the list of index form i to j is a palindrome;
        let mut dp = vec![vec![true; n]; n];
        // Record the tuple of the of `dp[][]` index;
        let mut res = (0, 0);

        for k in 1..n {
            for i in 0..(n - k) {
                if k == 1 {
                    // Initialization the dp[i][i+1];
                    // If s[i]==s[i+1] dp[i][i+1]=true,and that means the palindrome is even;
                    // else dp[i][i+1]=false;
                    dp[i][i + k] = s[i] == s[i + 1];
                } else {
                    // Record all of the palindrome;
                    // The k is the legth of palindrome;
                    dp[i][i + k] = (s[i] == s[i + k]) && dp[i + 1][i + k - 1];
                }
                if dp[i][i + k] {
                    // Record the index;
                    // The last of item,k is biggest;
                    res = (i, i + k);
                }
            }
        }
        // Convert the `Vec<char>` to `String`;
        s[res.0..=res.1].into_iter().collect::<String>()
    }
}
```
