use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let n1 : usize = tokens.next().unwrap().parse().unwrap();
    let n2 : usize = tokens.next().unwrap().parse().unwrap();
 
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let a : Vec<i32> = buffer.trim().split_whitespace().take(n1)
                                .map(|x| x.parse().unwrap())
                                .collect();

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let b : Vec<i32> = buffer.trim().split_whitespace().take(n2)
                                .map(|x| x.parse().unwrap())
                                .collect();

    let mut is_bubun = false;

    for i in 0..n1 {
        if a[i] == b[0] {
            for j in 0..n2 {
                if i + n2 - 1 >= n1 {
                    break;
                }

                if a[i + j] == b[j] {
                    is_bubun = true;
                } else {
                    is_bubun = false;
                    break;
                }
            }
        }

        if is_bubun {
            break;
        }
    }

    println!("{}", if is_bubun { "Yes" } else { "No" });
}