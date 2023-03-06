













use std::fs::File;
use std::io::prelude::*;

fn main() {

    // Open file     
    let mut file = File::open("test.txt").expect("Can't open file!");

    // Get file contents 
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File contents can't be read into string");

    // Print file contents 
    println!("File contents: {}", contents);



    // Get set of unique chars from contents
    let mut chars:Vec<char> = contents.chars().collect();
    chars.sort();
    chars.dedup();

    let len = chars.len();
}   
