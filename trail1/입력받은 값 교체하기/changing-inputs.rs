use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 불가");
    let mut input = input.split_whitespace();

    let mut a : i16 = input.next().expect("첫 번째 값이 없음")
                    .parse().expect("정수가 아니거나 값이 큼");
    let mut b : i16 = input.next().expect("두 번째 값이 없음")
                    .parse().expect("정수가 아니거나 값이 큼");

    (a, b) = (b, a);

    println!("{} {}", a, b);
}