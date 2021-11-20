// Vectors are resizable arrays
// import libray
use std::mem;

pub fn run() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Re-assign value
    numbers[2] = 20;
    println!("Single Value mutable: {}", numbers[2]);
    // Add on to vector
    numbers.push(5);
    numbers.push(6);
    println!("After push {:?}", numbers);

    numbers.pop();
    println!("After Pop {:?}", numbers);

    // Get Vector length
    println!("Vector length: {}", numbers.len());

    // Vector are stack allocated
    // print without importing libray
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
    // print with importing libray
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice from Vector
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x)
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers)
    

}