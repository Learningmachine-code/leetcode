fn main() {
    let n = 5;
    let mut a = vec![vec![true; n]; n];
    a[n - 2][n - 2] = false;
    for i in 0..n {
        for j in 0..n {
            print!("{} ", a[i][j]);
        }
        print!("\n");
    }
}
