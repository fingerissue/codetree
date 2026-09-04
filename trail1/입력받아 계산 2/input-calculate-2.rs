use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 실패");

    let mut tokens = input.split_whitespace();
    let a: i32 = tokens.next().expect("첫 번째 값이 없습니다").parse().expect("숫자가 아닙니다");
    let b: i32 = tokens.next().expect("두 번째 값이 없습니다").parse().expect("숫자가 아닙니다");
    
    println!("{}", a * b);
}
