fn main() {
    // fixed length string
    let string: &str = "fixed length string";
    println!("{}", string);

    // variable length string
    let mut vstring = String::from("variable length, ");
    vstring.push_str("string");
    vstring.push('s');
    println!("{}", vstring);
}
