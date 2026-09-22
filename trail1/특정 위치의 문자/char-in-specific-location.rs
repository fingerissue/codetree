use std::io;

fn main () {
    let lebros : [char; 6] = ['L', 'E', 'B', 'R', 'O', 'S'];

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input : char = buffer.trim().parse().unwrap();

    match lebros.iter().position(|&c| c == input) {
        Some(i) => println!("{i}"),
        None => println!("None"),
    }
}