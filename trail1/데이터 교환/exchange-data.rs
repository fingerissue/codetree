fn main() {
    let mut a : i8 = 5;
    let mut b : i8 = 6;
    let mut c : i8 = 7;

    let temp : i8 = b;
    b = a;
    a = c;
    c = temp;

    println!("{}\n{}\n{}", a, b, c);
}