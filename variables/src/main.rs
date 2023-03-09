fn main() {
    println!("variables");
    // basic types
    let i: i32 = 15;
    let f: f64 = 15.0;
    println!("this is an integer: {}", i);
    println!("this is a floating number: {}", f);
    // mutability - by default variables are immutable
    let mut m: i32 = 1;
    println!("m is: {}", m);
    m = 10;
    println!("m is: {}", m);
}
