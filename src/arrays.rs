// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn _run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Get array length
    println!("Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..];
    println!("Slice: {:?}", slice);
}