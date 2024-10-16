fn main() {
    let s = String::from("abasdadsbsddf");
    let v = &s.chars().collect::<Vec<_>>();
    let a = 3usize;
    println!("{}", v[a]);
}
