use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("입력 실패");
    let mut tokens = buffer.trim().split('.');

    let y : u16 = tokens.next().expect("년도 입력값이 없음")
                        .parse().expect("숫자가 아니거나 너무 큼");
    let m : u8 = tokens.next().expect("월 입력값이 없음")
                        .parse().expect("숫자가 아니거나 너무 큼");
    let d : u8 = tokens.next().expect("일 입력값이 없음")
                        .parse().expect("숫자가 아니거나 너무 큼");

    println!("{}-{}-{}", m, d, y);
}