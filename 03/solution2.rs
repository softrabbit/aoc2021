// https://adventofcode.com/2021/day/3

// Part 2: Way too much time spent on wrapping my head around
//         filters and such... and the logic is flawed too, for now.

use std::io::{self, BufRead};

// Count bit in position 'index' in supplied binary strings
fn count_bits(index: usize, binstrs: &Vec<String>) -> i32 {
    binstrs
        .iter()
        .map(|s| if s
             .chars()
             .nth(index)
             .unwrap() == '1' {
                 1
             } else {
                 -1
             })
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let mut data = Vec::new();
    for line in stdin.lock().lines() {
        let lstr = line.unwrap(); // Can only unwrap once because ownership
        data.push(lstr.clone());
    }
    
    // Here the counters are >0 for more ones than zeros and vice versa
    // Equal number selects 1
    let mut data_tmp = data.clone();

    for i in 0..data[0].len() {
        if data_tmp.len() > 1 {
            let c = count_bits(i, &data_tmp);

            let val = if c >= 0 { '1' } else { '0'};
            // println!("{}",c);
            data_tmp = data_tmp
                .into_iter() // NB iter() doesn't work here...
                .filter(|item| item.chars().nth(i).unwrap() == val )
                .collect();
            // println!("{:?} {:?} {:?}",i,c,data_tmp);
        }
    }

    // We hope there is only one string left...
    let oxygen = isize::from_str_radix(&data_tmp[0], 2).unwrap();

    
    // And then the same stuff for the _least_ seen digits, so the logic is reversed
    // These could really be in one loop
    data_tmp = data.clone();

    for i in 0..data[0].len() {
        if data_tmp.len() > 1 {
            let c = count_bits(i, &data_tmp);

            let val = if c >= 0 { '0' } else { '1'};
            // println!("{}",c);
            data_tmp = data_tmp
                .into_iter() // NB iter() doesn't work here...
                .filter(|item| item.chars().nth(i).unwrap() == val )
                .collect();
        }
    }

    let co2 = isize::from_str_radix(&data_tmp[0], 2).unwrap();
    println!("");

    println!("The number is {:?} ({} * {})", oxygen * co2, oxygen, co2);
}
