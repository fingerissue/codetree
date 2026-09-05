use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("입력 실패");
    let mut tokens = buffer.trim().split_whitespace();

    let h : i16 = tokens.next().expect("첫 번째 입력값이 없습니다")
                        .parse().expect("숫자가 아니거나 값이 너무 큽니다");
    let w : i16 = tokens.next().expect("두 번째 입력값이 없습니다")
                        .parse().expect("숫자가 아니거나 값이 너무 큽니다");

    let h = h as f64;
    let w = w as f64;
    
    let b = (10000.0 * w) / (h * h);

    println!("{}", b.floor());
    if b >= 25.0 {
        println!("Obesity");
    }
}