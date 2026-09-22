use std::io::{self, Read};

fn main () {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();
    
    let mut a : i32 = tokens.next().unwrap().parse().unwrap();
    let mut b : i32 = tokens.next().unwrap().parse().unwrap();
    let mut reminders : [i32; 10] = [0; 10];
    let mut result = 0;

    while a > 1 {
        let reminder = a % b;
        reminders[reminder as usize] += 1;

        a /= b;
    }

    for reminder in reminders {
        result += reminder * reminder;
    }
    println!("{result}");
}