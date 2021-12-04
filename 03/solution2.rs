// https://adventofcode.com/2021/day/3

// Part 2: Way too much time spent on wrapping my head around
//         filters and such... and the logic is flawed too, for now.

use std::io::{self, BufRead};
// use std::option::Option;

// Filters on the binstr matching value in position index...
fn myFilter(index: usize, value: char, binstr: &str) -> bool {
    if binstr.chars().nth(index).unwrap() == value {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut counters = Vec::new();
    let mut data = Vec::new();
    let mut binary;
    for line in stdin.lock().lines() {
        let lstr = line.unwrap(); // Can only unwrap once because ownership
        data.push(lstr.clone());
        binary = lstr.chars();
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
    // Here the counters are >0 for more ones than zeros and vice versa
    let mut data_tmp = data.clone();
    for (index, c) in counters.iter().enumerate() {
        if data_tmp.len() > 1 {
            let val = if *c > 0 { '1' } else { '0'};
            // println!("{}",c);
            data_tmp = data_tmp
                .into_iter() // NB iter() doesn't work here...
                .filter(|item| item.chars().nth(index).unwrap() == val )
                .collect();            
        }        
    }
    // We hope there is only one string left...
    let oxygen = isize::from_str_radix(&data_tmp[0], 2).unwrap();

    // And then the same stuff for the _least_ seen digits
    data_tmp = data.clone();
    for (index, c) in counters.iter().enumerate() {        
        print!("{} ",c);
        if data_tmp.len() > 1 {
            let val = if *c < 0 { '1' } else { '0'};
            data_tmp = data_tmp
                .into_iter() // NB iter() doesn't work here...
                .filter(|item| item.chars().nth(index).unwrap() == val )
                .collect();            
        }        
    }
    // We hope there is only one string left...
    let co2 = isize::from_str_radix(&data_tmp[0], 2).unwrap();
    println!("");
    
    // for line in data_tmp {
    //     println!("{}",line);
    // }
    // println!("{}",data_iter.count());

    

    println!("The number is {:?} ({} * {})", oxygen * co2, oxygen, co2);
}
