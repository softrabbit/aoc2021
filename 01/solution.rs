
// https://adventofcode.com/2021/day/1
// (count the number of times a depth measurement increases from the previous measurement)
// Reads from stdin and prints a number.
use std::io::{self, BufRead};
use std::option::Option;

fn main() {
    let mut last: Option<i32> = None;
    let mut val;
    let mut counter = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        val = line.unwrap().parse::<i32>().unwrap();
        if last.is_some() && val > last.unwrap() {
            counter += 1;
        }
        last = Some(val);
    }

    println!("The number is {:?}", counter);
}