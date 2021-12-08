// https://adventofcode.com/2021/day/8

// Decode 7-segment display wirings.

use std::io::{self, BufRead};

fn main() {
    // Set up the input reading and read input
    let stdin = io::stdin();    
    // Peekable might be useful, depending on how we iterate.
    let input_iter = stdin.lock().lines().peekable();
   
    // Input is multiline, keep in vector if needed later
    let mut input = Vec::new();
    for line in input_iter {
        input.push(line.unwrap());
    } 

    // Part 1, how many times do digits 1, 4, 7 or 8 appear?
    let mut counter = 0;
    for i in &input {
        let mut parts = i.split(" | ");
        let mut signals = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        let mut digits = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        // Digits have 2, 3, 4 or 7 segments lit
        for d in digits {
            if (d.len() > 1 && d.len() < 5) || d.len() == 7 {
                counter +=1;
            }
        }   
    }
    println!("First part says: {:?}", counter);

    // Part 2, actual decoding of patterns...
    // Patterns for each digit
    let patterns = ["abcefg", "cf", "acdeg", "acdfg","bcdf","abdfg", "abdefg", "acf","abcdefg","abcdfg"];
    counter = 0;
    for i in &input {
        // First, all options are equally possible
        let mut candidates = ["abcdefg"; 7];
        let mut parts = i.split(" | ");
        let mut signals = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        let mut digits = parts.next().unwrap().split(" ").collect::<Vec<&str>>();

        // 4 digits to decode for each line
        let mut code: [Option<i32>; 4] = [None, None, None, None];
             
        // First the easy ones...
        for index in 0..4 {
            if code[index].is_none() {
                if digits[index].len() == 2 { code[index] = Some(1); }
                if digits[index].len() == 3 { code[index] = Some(7); }
                if digits[index].len() == 4 { code[index] = Some(4); }
                if digits[index].len() == 7 { code[index] = Some(8); }
            }
        }
        println!("First pass: {:?}  {:?}", digits,code);

        if code[0].is_some() &&  code[1].is_some() && code[2].is_some() && code[3].is_some() {
            counter += code[0].unwrap()*1000 + code[1].unwrap()*100 + code[2].unwrap()*10 + code[3].unwrap();            
        } else {
            println!("Failed to compute {:?}", digits);
        }
    }  
    
}
