# Use the `vec![]`

```rust
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
```

```rust
cargo run
   Compiling a v0.1.0 (/home/a2580ti/code/leetcode/a)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/a`
true true true true true 
true true true true true 
true true true true true 
true true true false true 
true true true true true 
```
