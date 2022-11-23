// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify 
// or own string data

pub fn run() {
    let mut hello = String::from("Hello");

    // Get lengh
    println!("Length: {}", hello.len());

    // Push char
    hello.push(' ');

    // Push string
    hello.push_str("My People");
    println!("{}", hello);

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if is empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains: {}", hello.contains("H"));

    // Replace
    println!("Replace: {}", hello.replace("My People", "Person"));

    // Loop throught string by whitespace
    println!("");
    for word in hello.split_whitespace() {        
        println!("{}", word);
    }

    // Create string with capacity
    println!("");
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());

    println!("");
    println!("{}", hello);
}