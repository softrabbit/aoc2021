
// https://adventofcode.com/2021/day/1
// Part 2: count the number of times a 3-element sliding sum is larger than the previous

// Reads from stdin and prints a number.
use std::io::{self, BufRead};
use std::option::Option;

fn main() {
    
    let mut buffer: [Option<i32>; 3] = [None; 3];
    let mut ptr = 0; // Where in the buffer we're working

    let mut last: Option<i32> = None;
    let mut val;
    let mut counter = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Read and stuff into buffer
        val = line.unwrap().parse::<i32>().unwrap();
        buffer[ptr] = Some(val);
        ptr += 1;
        ptr %= 3;

        if buffer[0].is_some() && buffer[1].is_some() && buffer[2].is_some() {
            let sum = buffer[0].unwrap() + buffer[1].unwrap() + buffer[2].unwrap();
            if last.is_some() && sum > last.unwrap() {
                counter += 1;
            }
            last = Some(sum);
        }
        
        
    }

    println!("The number is {:?}", counter);
}