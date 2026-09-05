use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("입력 실패");
    let mut tokens = buffer.trim().split_whitespace();

    let a : i16 = tokens.next().expect("첫 번째 입력값이 없음")
                        .parse().expect("숫자가 아니거나 너무 큼");
    let b : i16 = tokens.next().expect("두 번째 입력값이 없음")
                        .parse().expect("숫자가 아니거나 너무 큼");
    
    println!("{}\n{}\n{}\n{}", a + b, a - b, a / b, a % b);
}