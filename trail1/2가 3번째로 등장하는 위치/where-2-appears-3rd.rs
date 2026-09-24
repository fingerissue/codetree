use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : usize = buffer.trim().parse().unwrap();

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let arr : Vec<i32> = buffer.trim().split_whitespace().take(n)
                                .map(|x| x.parse().unwrap())
                                .collect();
    
    let mut slice = &arr[..];
    let mut res = 0;
    for _ in 0..3 {
        match slice.iter().position(|&x| x == 2) {
            Some(i) => {
                res += i + 1;
                slice = &slice[i + 1..];
            },
            None => println!("Not here"),
        }
    }
    
    println!("{res}");
}