use std::{io, collections::HashMap};

fn main() {
    println!("Enter a string to check for unique characters");

    let mut input = String::new().to_uppercase();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let new_input = input.trim().to_uppercase();

    if has_unique_characters(&new_input) {
        println!("The string has all unique characters");
    } else {
        println!("The string does not have all unique characters");
    }
}

fn has_unique_characters(input: &str) -> bool {
    let mut char_count = HashMap:: new();

    for c in input.chars() {
        let count = char_count.entry(c).or_insert(0);

        *count +=1;

        if *count > 1 {
            return  false;
        }
    } 
true
}