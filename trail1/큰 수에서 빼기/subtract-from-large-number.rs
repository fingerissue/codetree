use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("입력 실패");
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().expect("첫 번째 입력값이 없습니다")
                        .parse().expect("숫자가 아니거나 너무 큽니다");
    let b : i8 = tokens.next().expect("두 번째 입력값이 없습니다")
                        .parse().expect("숫자가 아니거나 너무 큽니다");

    if (a > b) {
        println!("{}", a - b);
    } else {
        println!("{}", b - a);
    }
}