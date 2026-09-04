use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("입력 불가");
    let s : &str = s.trim();

    println!("{}", s);
}