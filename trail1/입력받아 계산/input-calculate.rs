use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("입력 실패했습니다.");
    
    let mut a : i8 = a.trim().parse().expect("숫자가 아닙니다.");

    a = a + 2;

    println!("{}", a);
}