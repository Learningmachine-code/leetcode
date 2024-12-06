pub fn convert(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    if num_rows == 1 {
        return s;
    }
    let mut result = String::new();
    let s_chars: Vec<char> = s.chars().collect();
    let cycle_length = 2 * num_rows - 2;
    let mut index_list = Vec::new();
    for row in 0..num_rows {
        let mut index = row;
        while index < s_chars.len() {
            result.push(s_chars[index]);
            index_list.push(index);
            if row != 0 && row != num_rows - 1 {
                // Key line:
                let inner_index = index + cycle_length - 2 * row;
                if inner_index < s_chars.len() {
                    result.push(s_chars[inner_index]);
                    index_list.push(inner_index);
                }
            }
            index += cycle_length;
        }
    }
    println!("{:?}", index_list);
    result
}
