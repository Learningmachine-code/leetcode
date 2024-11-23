mod leetcode;
fn main() {
    let input = String::from("asdfghqwerrewqadsvedbesvgb");
    let result = leetcode::s5::s5_1::longest_palindrome(input);
    println!("The result is {result}");
}
