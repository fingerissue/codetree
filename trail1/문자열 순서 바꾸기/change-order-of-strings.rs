use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("입력 실패");
    let mut buffer = buffer.split_whitespace();

    let s : &str = buffer.next().expect("첫 번째 입력값 없음");
    let t : &str = buffer.next().expect("두 번째 입력값 없음");
                        
    println!("{}\n{}", t, s);
}