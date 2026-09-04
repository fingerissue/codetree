use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("입력 실패");
    let mut a : i16 = a.trim().parse().expect("숫자가 아니거나 너무 큼");

    a = a * 2 + 3;

    println!("{}", a);
}