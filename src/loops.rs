pub fn _run() {
    let mut counter = 0;
    let numbers: &[i32; 10] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    loop {
        println!("Number: {}", numbers[counter]);
        // Inc
        counter += 1;

        if counter == numbers.len() {
            break;
        }
    }

    println!("\n");

    // While Loop (FizzBuzz)
    while counter <= 100 {
        if counter % 15 == 0 {
            println!("fizzbuzz");
        } else if counter % 3 == 0 {
            println!("fizz");
        } else if counter % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", counter);
        }

        // Inc
        counter += 1;
    }

    println!("\n");

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }    

    println!("\n");

    for mut x in 0..numbers.len() {
        x += 2;
        println!("{}", x);
    }

}
