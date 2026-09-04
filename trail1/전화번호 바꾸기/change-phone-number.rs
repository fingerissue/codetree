use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("입력 실패");
    let mut tokens = buffer.trim().split('-');

    let _ = tokens.next();
    let x : &str = tokens.next().expect("첫 번째 입력값이 없음");
    let y : &str = tokens.next().expect("두 번째 입력값이 없음");
    
    println!("010-{}-{}", y, x);
}