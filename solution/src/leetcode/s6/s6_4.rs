pub fn convert(s: String, num_rows: i32) -> String {
    // 为什么输入类型总是i32
    let num_rows = num_rows as usize;
    // 每行一个String
    let mut rows = vec![String::new(); num_rows];
    // z字形往复的迭代器，01232101232......
    let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
    // 按迭代器给出的下标访问对应行，推入字符
    iter.zip(s.chars()).for_each(|(i, c)| rows[i].push(c));
    // collect连接每行
    rows.into_iter().collect()
}
