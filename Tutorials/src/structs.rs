// Structs - Used to create custom data types


// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8

}

// Tuple Struct
struct TupleColor(u8, u8, u8);

// Struct with functions
struct Person {
    first_name: String,
    last_name: String

}
impl Person{
    // Construct Person
    fn new(first: &str, last: &str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()

        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
pub fn run() {
    // Traditional Struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0

    };
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Change values

    c.red = 200;

    println!("Value Changed Color: {} {} {}", c.red, c.green, c.blue);


    // Tuple Structs
    let mut tc = TupleColor(255, 0, 0);
    println!("Tuple Color: {} {} {}", tc.0, tc.1, tc.2);
    tc.1 = 150;
    println!("Tuple Color Changed: {} {} {}", tc.0, tc.1, tc.2);

    // Function Struct
    let mut p = Person::new("Mary", "Doe");
    
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("Jane");
    println!("Person {}", p.full_name());
    println!("Person tuple {:?}", p.to_tuple());

}