fn main() {
    let mut a : i8 = 5;
    let mut b : i8 = 6;
    let mut c : i8 = 7;

    (a, b, c) = (c, c, c);

    println!("{} {} {}", a, b, c);
}