pub fn run() {
    // Print to console
    println!("Hello from the print rs file");

    // Basic Formatting
    println!("{} have to put in curly brackets to print integers and to format {} you dont have to specify a list {}", "you", 1, "to import");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name="John", activity="Baseball");

    //Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
    
}