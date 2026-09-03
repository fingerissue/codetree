fn main() {
    let mut a : i8 = 1;
    let mut b : i8 = 2;
    let mut c : i8 = 3;

    a = c;
    b = a;

    println!("{} {} {}", a, b, c);
}