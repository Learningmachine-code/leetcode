impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let len = s.len();
        //println!("the string`s len={len}");
        let s: Vec<char> = s.chars().collect();
        let num_len = (len as i32 / (2 * num_rows - 2) + 1) * (num_rows - 1);
        let mut num_chart = vec![vec!['A'; num_len as usize]; num_rows as usize];
        let num_count = len as i32 / (2 * num_rows - 2) + 1;
        let mut k: usize = 0;
        let mut j: usize = 0;
        for _ in 0..num_count {
            for i in 0..num_rows {
                if k == len {
                    break;
                }
                num_chart[i as usize][j] = s[k];
                k += 1;
            }
            let mut m = num_rows as usize - 1;
            for _ in 0..num_rows - 2 {
                if k == len {
                    break;
                }
                j += 1;
                m -= 1;
                num_chart[m][j] = s[k];
                k += 1;
            }
            j += 1;
        }
        //print_chart(num_chart.clone(), num_rows, num_len);
        num_convert(num_chart, num_rows, num_len, len)
    }

    fn num_convert(chart: Vec<Vec<char>>, num_rows: i32, num_len: i32, len: usize) -> String {
        let mut a = vec!['A'; len as usize];
        let mut k: usize = 0;
        let num_len = num_len as usize;
        let num_rows = num_rows as usize;
        for i in 0..num_rows {
            if k == len {
                break;
            }
            for j in 0..num_len {
                if chart[i][j] == 'A' {
                    continue;
                } else {
                    if k == len {
                        break;
                    } else {
                        a[k] = chart[i][j];
                        k += 1;
                    }
                }
            }
        }

        a[0..len].iter().collect::<String>()
    }

    fn print_chart(chart: Vec<Vec<char>>, num_rows: i32, num_len: i32) {
        for i in 0..num_rows {
            for j in 0..num_len {
                if chart[i as usize][j as usize] == 'A' {
                    print!("\u{c84} ");
                } else {
                    print!("{} ", chart[i as usize][j as usize]);
                }
            }
            print!("\n");
        }
    }
}
