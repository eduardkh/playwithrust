fn main() {
    // fixed length string
    let string: &str = "fixed length string";
    println!("{}", string);

    // variable length string
    let mut vstring = String::from("variable length, ");
    vstring.push_str("string");
    vstring.push('s');
    println!("{}", vstring);

    // string operations
    println!("vstring empty: {}", vstring.is_empty());
    println!("vstring len: {}", vstring.len());
    println!("vstring byte capacity: {}", vstring.capacity());
    println!("vstring contains 'rings': {}", vstring.contains("rings"));
    // multiple string operations
    vstring.push_str("    ");
    println!(
        "vstring after adding spaces, trim and len: {}",
        vstring.trim().len()
    );
}
