use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut arr : [i32; 10] = [0; 10];

    for i in 0..10 {
        let token : i32 = tokens.next().unwrap().parse().unwrap();
        arr[i] = token;
    }

    let sum_zzak = arr[1] + arr[3] + arr[5] + arr[7] + arr[9];
    let sum_sam = arr[2] + arr[5] + arr[8];
    let mean_sam = sum_sam as f64 / 3 as f64;
    
    println!("{sum_zzak} {mean_sam:.1}");
}