use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 불가");
    let mut input = input.split_whitespace();

    let a : i16 = input.next().expect("첫 번째 값이 없습니다")
                        .parse().expect("숫자가 아닙니다");
    let b : i16 = input.next().expect("두 번째 값이 없습니다")
                        .parse().expect("숫자가 아닙니다");
    
    println!("{}", a + b);
}