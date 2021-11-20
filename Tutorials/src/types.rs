// Rust is a staticallyy typed language, which means that it must know the types of all variables at compile time, however, 
// the compiler can usually infer what type we want to use based on the values and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 12341234;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max f64: {}", std::f64::MAX);

    //Boolean
    let is_active = true;
    // let is_active: bool = true;
    
    // Get boolean from expression
    let is_greater = 10 > 5;

    // Character
    let a1 = 'a';

    // Emoji unicode character
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));


    
}