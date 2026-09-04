use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("입력 오류");
    let a : i16 = a.trim().parse().expect("숫자가 아님");

    println!("Your score is {} point.", a);
}