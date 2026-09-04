use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("입력 불가");
    let mut a : f32 = a.trim().parse().expect("변환 불가");

    a += 1.5;

    println!("{}", a);
}