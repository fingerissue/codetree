use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let n : i32 = tokens.next().unwrap().parse().unwrap();
    let q : i32 = tokens.next().unwrap().parse().unwrap();

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let arr : Vec<i32> = buffer.trim().split_whitespace().take(n as usize)
                    .map(|token| token.parse().unwrap()).collect();

    for _ in 0..q {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut tokens = buffer.trim().split_whitespace();
        
        let mode : i32 = tokens.next().unwrap().parse().unwrap();
        let arg1 : i32 = tokens.next().unwrap().parse().unwrap();

        match mode {
            1 => println!("{}", arr[(arg1 - 1) as usize]),
            2 => match arr.iter().position(|&x| x == arg1) {
                Some(i) => println!("{}", i + 1),
                None => println!("0"),
            },
            3 => {
                let arg2 : i32 = tokens.next().unwrap().parse().unwrap();
                for i in arg1 - 1..arg2 {
                    print!("{} ", arr[i as usize]);
                }
                print!("\n");
            },
            _ => println!("Not here"),
        }
    }
}