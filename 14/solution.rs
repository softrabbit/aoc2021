// https://adventofcode.com/2021/day/14

// Exponential growth thingy. Part 1 can be brute forced easily.

use std::io::{self, BufRead};
use std::collections::HashMap;

fn evolve(polymer: String, rules: &HashMap<String, String> ) -> String {
    let mut next = Vec::<String>::new();
    // Take 2 characters, check if in rules and if so, apply rule
    for i in 0..polymer.len()-1 {
        let candidate = &polymer[i..i+2];
        if rules.contains_key(candidate) {
            next.push(candidate[0..1].to_string());
            next.push(rules.get(candidate).unwrap().to_string());
        } else {
            next.push(candidate[0..1].to_string());
        }
    }
    // And finally the last character
    next.push(polymer[polymer.len()-1..polymer.len()].to_string());

    return next.iter().map(|s| s.to_string()).collect::<String>();
}

fn main() {
    // Set up the input reading and read input
    let stdin = io::stdin();    
    // Peekable might be useful, depending on how we iterate.
    let mut input_iter = stdin.lock().lines().peekable();

    // First line holds the starting template
    let template = input_iter.next().unwrap().unwrap();
    // Skip a blank
    let _ = input_iter.next();


    let mut rules = HashMap::new();
    let mut tmp;
    for line in input_iter {
        tmp = line
            .unwrap()
            .split(" -> ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        // So we get the rules in tuples like ("NN", "C")
        rules.insert( tmp[0].clone(), tmp[1].clone() );
    } 

    let mut polymer = template;
    // This is fine for part 1, but part 2 needs something smarter
    // than generating the entire thing...
    // I'm getting a crash like this when counting to 40:
    // 18 9961473
    // memory allocation of 144115188075855873 bytes failed
    for n in 0..10 {
        polymer = evolve(polymer, &rules);
        println!("{:?} {:?}", n, polymer.len());
        
    }
    // Take the quantity of the most common element and
    // subtract the quantity of the least common element?
    let result = 0;
    let mut counts = HashMap::<char, i64>::new();
    for c in polymer.chars() {
        if counts.contains_key(&c) {
            let n = counts.get_mut(&c).unwrap();
            *n += 1;
        } else {
            counts.insert(c, 1);
        }
    }
    let vals: Vec<i64> = counts.into_values().collect();
    let result = vals.iter().max().unwrap() - vals.iter().min().unwrap();
    println!("The number is {:?}", result);
}
