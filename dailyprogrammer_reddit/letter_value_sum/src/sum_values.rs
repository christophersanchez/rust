use std::fs::File;
use std::io::prelude::*;


fn lettersum(path: &str) {

    let mut file = File::open(path).expect("Can't open file.");
    let mut contents = String::new();
    let mut odd_numbers: Vec<&str> = vec![];
    file.read_to_string(&mut contents).expect("Can't read file.");
    for word in contents.split("\n") {
        // println!("{}", word);
        let word: &str = word;

        let mut sum_letters: Vec<u32> = vec![];
        for letter in word.chars() {
            // println!("{}", letter);
            let ch = letter;
            let num: u32 = ch.to_digit(36).unwrap() - 9;
            sum_letters.push(num);
        }
        
        let sum: u32 = sum_letters.iter().sum();

        if &sum % 2 == 0 {
            // println!("{} even", &word);
            continue

        } else if sum == 319 {
            println!("{} equals 319", word);
            odd_numbers.push(&word);
        } else {
            odd_numbers.push(&word);
        }


    }
    println!("There are {:?} words with odd sums.",  odd_numbers.iter().count());


}

pub fn run() {
    println!("{:?}", lettersum("/Users/thestrawheckergroup/Desktop/Rust/dailyprogrammer_reddit/letter_value_sum/src/enable1.txt"));

}