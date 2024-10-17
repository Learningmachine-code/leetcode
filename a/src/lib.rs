pub struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
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
    }
}
