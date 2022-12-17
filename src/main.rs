fn main() {
    println!("Hello, world!");

    let guess: i64 = "42".parse().expect("not a number");
    print!("{guess}");
}
