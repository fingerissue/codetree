use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let arr : Vec<i32> = buffer.trim().split_whitespace().take(10)
                                .map(|x| x.parse().unwrap())
                                .collect();
    
    
    let mut max = arr[0];

    for &val in &arr[1..] {
        if val > max {
            max = val;
        }
    }

    println!("{max}");
}