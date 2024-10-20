use testcode::Solution;
fn main() {
    let input = String::from("asdfghqwerrewqadsvedbesvgb");
    let result = Solution::logest_palindrome(input);
    println!("The result is {result}");
}
