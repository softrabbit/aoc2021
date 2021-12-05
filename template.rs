// https://adventofcode.com/2021/day/X

// Describe the problem briefly here

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let result;
    let mut input = Vec::new();
    
    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }

    // Logic goes here
    

    println!("The number is {:?}", result);
}
