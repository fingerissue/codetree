use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("입력 실패");
    let a : i8 = buffer.trim().parse().expect("숫자가 아니거나 너무 큼");

    println!("{}", a);

    if (a < 0) {
        println!("minus");
    }
}