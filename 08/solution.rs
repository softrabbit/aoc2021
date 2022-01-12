// https://adventofcode.com/2021/day/8

// Decode 7-segment display wirings.
// Input is lines of 10 patterns corresponding to the digits and a 4-pattern code:
// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce

//    0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg
// 

use std::io::{self, BufRead};
use std::collections::HashSet;

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
        let _signals = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        let digits = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        // Digits we count have 2, 3, 4 or 7 segments lit
        for d in digits {
            if (d.len() > 1 && d.len() < 5) || d.len() == 7 {
                counter +=1;
            }
        }   
    }

    // Part 2, actual decoding of patterns...
    counter = 0;
    for i in &input {
        let mut parts = i.split(" | ");
        let mut tmp = parts.next().unwrap().split(" ").collect::<Vec<&str>>();

        // Make two sets, one for the signals 0-10...
        let mut signals = Vec::new();
        for x in tmp.iter() {            
            signals.push( x.chars().collect::<HashSet<char>>() );
        }

        // ... and one for the digits in the code
        let mut digits = Vec::new();
        tmp = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        for x in tmp.iter() {
            digits.push( x.chars().collect::<HashSet<char>>() );
        }

        // Keep track of index for each digit in signals
        let mut digit_indexes: [Option<usize>; 10] = [None; 10];          

        // First, find the easy ones (1,4,7,8) in the "signals" list
        for index in 0..10 {
            if signals[index].len() == 2 { digit_indexes[1] = Some(index); }
            if signals[index].len() == 3 { digit_indexes[7] = Some(index); }
            if signals[index].len() == 4 { digit_indexes[4] = Some(index); }
            if signals[index].len() == 7 { digit_indexes[8] = Some(index); }
        }
        
        // - out of the three 6-segment ones:
        //    - 6 lacks one of the segments in 1
        //    - of the remaining 2, 0 lacks one of the segments in 4
        //    - the third one is 9
        for index in 0..10 {
            if signals[index].len() == 6 {
                if signals[index].intersection(&signals[digit_indexes[1].unwrap()]).collect::<HashSet<&char>>().len() == 1 {
                    digit_indexes[6] = Some(index);
                } else if signals[index].intersection(&signals[digit_indexes[4].unwrap()]).collect::<HashSet<&char>>().len() == 3 {
                    digit_indexes[0] = Some(index);
                } else {
                    digit_indexes[9] = Some(index);
                }
            }
        }
        
        // - 5-segment ones (2,3,5):
        //    - 5 lacks one segment from 9 and one from 6, has no segments outside those
        //    - 3 lacks one, adds one compared to 5
        //    - 2 lacks two, adds two compares to 5
        for index in 0..10 {
            if signals[index].len() == 5 {
                let is6 = signals[index].intersection(&signals[digit_indexes[6].unwrap()]).collect::<HashSet<&char>>().len();
                let is9 = signals[index].intersection(&signals[digit_indexes[9].unwrap()]).collect::<HashSet<&char>>().len();
                if is6 == 5 && is9 == 5 {
                    digit_indexes[5] = Some(index);                    
                } else if is6 == 4 && is9 == 5 {
                    digit_indexes[3] = Some(index);
                } else if is6 == 4 && is9 == 4 {
                    digit_indexes[2] = Some(index);
                }
                
            }
        }

        // Last, figure the code out.
        let mut sum = 0;
        for i in 0..4 {
            // Find where the code digit is in the signals
            let index = signals.iter().position(|s| s == &digits[i]).unwrap();
            // Check which number index is pointing at that signal (we're going backwards here...)
            let num = digit_indexes.iter().position(|&n| n == Some(index)).unwrap();
            sum = sum * 10 + num;
        }
        counter += sum;
    }
    println!("{:?}",counter);
    
}
