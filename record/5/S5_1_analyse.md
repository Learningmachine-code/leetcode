# code analyse

```rust
mod leetcode;
fn main() {
    let input = String::from("asdfghqwerrewqadsvedbesvgb");
    let result = leetcode::s5::s5_1::longest_palindrome(input);
    println!("The result is {result}");
}

pub fn longest_palindrome(s: String) -> String {
    let len = s.len();
    //Convert the String to the Vec<char> that can be called by index.
    let s: Vec<char> = s.chars().collect();

    //Create a two-dimensional Vec to store the index of palindrome substrings.
    //If (2,5) is true ,then it means that substring form index 3 to 6 is a palindromic substring.
    let mut dp = vec![vec![true; len]; len];
    //Store the index of the largest palindromic substring.
    let mut res = (0, 0);

    //K mesns the length of the substrings.
    //The inner loop goes form 0 to (len-k) to prevent out of bounds.
    for k in 1..len {
        for i in 0..(len - k) {
            if k == 1 {
                //
                dp[i][i + k] = s[i] == s[i + 1];
            } else {
                dp[i][i + k] = (s[i] == s[i + k]) && dp[i + 1][i + k - 1];
            }
            //Because k loops form small to large,the res at the end is the largest substring.
            if dp[i][i + k] {
                res = (i, i + k);
            }
        }
    }
    //Convert the Vec to String.
    s[res.0..=res.1].iter().collect::<String>()
}
#[cfg(test)]
mod test {
    use super::longest_palindrome;
    #[test]
    fn tests() {
        let input = String::from("asdfghqwerrewqadsvedbesvgb");
        let result = longest_palindrome(input);
        println!("The result is {result}");
    }
}
/*
fn main() {
    let input = String::from("asdfghqwerrewqadsvedbesvgb");
    let result = leetcode::s5::s5_1::longest_palindrome(input);
    println!("The result is {result}");
}
*/
```
