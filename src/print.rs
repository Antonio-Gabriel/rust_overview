pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "AG", "Mass", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name="António Gabriel", activity = "piano"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:0}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, 23, 23));
}
