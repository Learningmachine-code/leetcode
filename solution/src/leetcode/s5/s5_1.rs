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
