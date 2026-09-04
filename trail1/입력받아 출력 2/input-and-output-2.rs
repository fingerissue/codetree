use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("입력 실패");
    let mut buffer = buffer.trim().split('-');

    let front : &str = buffer.next().expect("앞자리 입력 안 됨");
    let back : &str = buffer.next().expect("뒷자리 입력 안 됨");

    println!("{}{}", front, back);
}