
// https://adventofcode.com/2021/day/2

// Track movement in 2 dimensions from simple instructions
// Part 1: Depth changes straight from up/down instructions
// Part 2: Up/down changes "aim", that operates on depth when going forward

// Compile like 'rustc --cfg "part2" solution.rs' to get the 2nd algorithm



// Reads from stdin and prints a number.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut horizontal = 0;
    let mut depth = 0;
    #[cfg(part2)]
    let mut aim = 0;

    for line in stdin.lock().lines() {
        let lstr = line.unwrap();
        let n = lstr.split_whitespace().last().unwrap().parse::<i32>().unwrap();

        #[cfg(not(part2))]
        {
            if lstr.starts_with("forward") {
                horizontal += n;
            } else if lstr.starts_with("up") {
                depth -= n;
            } else if lstr.starts_with("down") {
                depth +=n;
            } else {
                // WTF?
            }
        }
        #[cfg(part2)]
        {
            if lstr.starts_with("forward") {
                horizontal += n;
                depth += n*aim;
            } else if lstr.starts_with("up") {
                aim -= n;
            } else if lstr.starts_with("down") {
                aim +=n;
            } else {
                // WTF?
            }
        }
    }

    println!("The number is {:?}", horizontal*depth );
}