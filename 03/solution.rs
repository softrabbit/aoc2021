
// https://adventofcode.com/2021/day/3

// Part 1: make a binary number out of most common bits in each column
//         and multiply with the same from least common bits.

use std::io::{self, BufRead};
// use std::option::Option;

fn main() {
    let stdin = io::stdin();
    let mut counters = Vec::new();
    for line in stdin.lock().lines() {
        let lstr = line.unwrap(); // Can only unwrap once because ownership
        let binary = lstr.chars();
        while lstr.len() > counters.len() {
            counters.push(0);
        }
        for (index, c) in binary.enumerate() {
            if c == '1' {
                counters[index] += 1;
            } else {
                counters[index] -= 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for n in counters {
        gamma *= 2;
        epsilon *= 2;
        if n > 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    

    println!("The number is {:?}", gamma * epsilon);
}
