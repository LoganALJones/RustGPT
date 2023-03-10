use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {

    // Open file 
    let mut file = File::open("test.txt").expect("Can't open file!");

    // Get file contents 
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File contents can't be read into string");

    // Get vocab: defines the set of possible characters that 
    // the model can generate.
    let mut vocab:Vec<char> = contents.chars().collect();
    vocab.sort();
    vocab.dedup();

    // Tokenize: convert characters into a sequence of integers.
    // Define encoding table: 
    let mut stoi = HashMap::new();
    let mut itos = HashMap::new();

    for (i, ch) in vocab.into_iter().enumerate() {
        stoi.insert(ch, i);
        itos.insert(i, ch);
    }
}

fn encoding(s: &str, h: &HashMap<char, usize>) -> Vec<usize> {
    s.chars().map(|c| *h.get(&c).unwrap()).collect()

}

fn decoding(v: Vec<usize>, itos: &HashMap<usize, char>) -> String {
   v.iter().map(|i| itos.get(i).unwrap()).collect() 
}