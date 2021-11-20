pub fn run() {
    // Primitive string = Immutable fixed-length string somewhere in memory
    // String = Growable, heap-allocated data structure - Use when you need to modify or own string data.
    let hello = "Hello";
    let mut world = String::from("World");
    println!("{}", hello);
    println!("{}", world);

    // Get Length
    println!("Length: {}", hello.len());
    println!("Length: {}", world.len());

    // Push char 
    world.push('w');

    // Push String
    world.push_str("orld");
    println!("{}", world);

    // Capacity in bytes
    println!("Capacity: {}", world.capacity());
    println!("Is empty: {}", hello.is_empty());
    println!("Is empty: {}", world.is_empty());

    // Contains substring
    println!("Contains 'World' {}", hello.contains("World"));
    println!("Contains 'World' {}", world.contains("World"));

    // Replace 
    println!("Replace: {}", hello.replace("Hello", "There"));
    println!("Replace: {}", world.replace("World", "There"));

    // Loop thrugh string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
    

}    