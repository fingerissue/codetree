use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("입력 실패");
    let mut buffer = buffer.split_whitespace();

    let a : i16 = buffer.next().expect("첫 번째 입력값 없음")
                        .parse().expect("숫자가 아니거나 너무 큼");
    let b : i16 = buffer.next().expect("두 번째 입력값 없음")
                        .parse().expect("숫자가 아니거나 너무 큼");
    
    println!("{} {}", a, b);
}