// Arrays - Fixed list where elements are the same data types
// Length is fixed

// import libray
use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // Re-assign value
    numbers[2] = 20;
    println!("Single Value mutable: {}", numbers[2]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    // print without importing libray
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    // print with importing libray
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice from array
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}", slice);


}