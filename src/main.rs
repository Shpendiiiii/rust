


fn main() {
    println!("Hello, world!");
    //parse converts string -> int
    let guess: i64 = "42".parse().expect("not a number");
    println!("{guess}");

   //tuples are fixed, line 10 is how you access an element from a tuple
    let tube: (u8, i16, f32) = (205, -250, 255.55);
    let first = tube.0;
    println!("tuple 2 first element {first}");
    // println!("tuple 1 {tup1}");
    println!("{}", i8::MAX);



        for number in (1..=4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");

}
