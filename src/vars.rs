// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn _run() {
    let name = "António";
    let mut age = 26;    

    println!("My name is: {name} and I'm {age}", name=name, age=age);
    age = 27;
    println!("I'm happy to complete more one years old, now I'm {}", age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (email, paswd) = ("antoniogabriel@gmail.com", "antoniocampos20");
    println!("My email is {0} and my password is {1}", email, paswd);
}