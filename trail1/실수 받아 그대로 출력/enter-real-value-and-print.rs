use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("입력 불가");
    let a : f32 = a.trim().parse().expect("변환 불가");

    println!("{:.2}", a);
}