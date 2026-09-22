use std::io::{self, Read};

fn main () {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut is_cold : [bool; 3] = [false; 3];
    let mut fevers : [i32; 3] = [0; 3];
    let mut level : [i32; 5] = [0; 5];

    for i in 0..3 {
        let mut cold : char = tokens.next().unwrap().parse().unwrap();
        let mut fever : i32 = tokens.next().unwrap().parse().unwrap();

        if cold == 'Y' {
            is_cold[i] = true;
        }
        fevers[i] = fever;
    }
    
    for i in 0..3 {
        if fevers[i] >= 37 {
            if is_cold[i] {
                level[0] += 1;
            } else {
                level[1] += 1;
            }
        } else {
            if is_cold[i] {
                level[2] += 1;
            } else {
                level[3] += 1;
            }
        }
    }

    if level[0] >= 2 {
        level[4] += 1;
    }

    for i in 0..4 {
        print!("{} ", level[i]);
    }
    
    if level[4] > 0 {
        print!("E");
    }
}