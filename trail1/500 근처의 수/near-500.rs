use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let arr : Vec<i32> = buffer.trim().split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .collect();

    let mut less_500 = 0;
    let mut more_500 = 1000;

    for var in arr {
        if var < 500 {
            if less_500 < var {
                less_500 = var;
            }
        } else if var > 500 {
            if more_500 > var {
                more_500 = var;
            }
        }
    }

    println!("{less_500} {more_500}")
}