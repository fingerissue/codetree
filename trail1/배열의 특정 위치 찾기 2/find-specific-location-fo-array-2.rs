use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut arr : [i32; 10] = [0; 10];
    let mut sum_hol = 0;
    let mut sum_zzak = 0;

    for i in 0..10 {
        let token : i32 = tokens.next().unwrap().parse().unwrap();
        arr[i] = token;

        if i % 2 == 0 {
            sum_hol += arr[i];
        } else {
            sum_zzak += arr[i];
        }
    }

    let cha = sum_hol.max(sum_zzak) - sum_zzak.min(sum_hol);
    println!("{cha}");
}