use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    let mut cnt_gyo : i16 = 0;
    let mut cnt_bok : i16 = 0;
    let mut cnt_wha : i16 = 0;
    
    for i in 1..=n {
        if i % 12 == 0 {
            cnt_wha += 1;
        } else if i % 3 == 0 {
            cnt_bok += 1;
        } else if i % 2 == 0 {
            cnt_gyo += 1;
        }
    }

    println!("{} {} {}", cnt_gyo, cnt_bok, cnt_wha);
}