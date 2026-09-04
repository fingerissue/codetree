use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("입력 실패");
    let mut buffer = buffer.trim().split('-');

    let a : i16 = buffer.next().expect("첫 번째 입력값이 없음")
                        .parse().expect("숫자가 아니거나 너무 큼");
    let b : i16 = buffer.next().expect("두 번째 입력값이 없음")
                        .parse().expect("숫자가 아니거나 너무 큼");
    let c : i16 = buffer.next().expect("세 번째 입력값이 없음")
                        .parse().expect("슷자가 아니거나 너무 큼");
                                                                        
    println!("{}.{}.{}", c, a, b);
}