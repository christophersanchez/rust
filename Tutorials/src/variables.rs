pub fn run() {
    // Immutable
    let name = "Brad";
    // Mutable
    let mut age = 37;
    age = 38;
    println!{"My name is {}, and I am {}", name, age};

    // Define constant
    const ID: i32 = 001; 
    println!("ID: {}", ID);

    // Assigne multiple variables at once
    let ( my_name, my_age) = ("Brad", "37");
    println!("{} is {}", my_name, my_age);

}