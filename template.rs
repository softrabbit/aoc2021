// https://adventofcode.com/2021/day/X

// Describe the problem briefly here

use std::io::{self, BufRead};

fn main() {
    // Set up the input reading and read input
    let stdin = io::stdin();    
    // Peekable might be useful, depending on how we iterate.
    let input_iter = stdin.lock().lines().peekable();
    // Input is a single line
    /*
    let line1 = input_iter.next().unwrap().unwrap();
    // Comma-separated list of numbers?
    let numbers: Vec<i64> = line1.split(',')
        .map(|s| s.parse::<i64>().unwrap() )
        .collect();
    */
    
    // Input is multiline, keep in vector if needed later
    /*
    let mut input = Vec::new();
    for line in input_iter {
        input.push(line.unwrap());
    } 
    */
    
    // Logic goes here
    let result = 0;

    println!("The number is {:?}", result);
}
