// https://adventofcode.com/2021/day/14

// Exponential growth thingy, part 2.
// For 10 iterations brute forcing is fine, for 40.. not so great.

use std::io::{self, BufRead};
use std::collections::HashMap;

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
    println!("{:?} rules loaded",rules.len());
    


    // All we need to keep in mind are counts of pairs, so:
    // NNCB -> NN, NC, CB (
    // NN -> NC, CN | NC -> NB, BC | CB -> CH, HB
    let mut counts = HashMap::<String, i64>::new();
    
    // Remember last char, it'll be useful later...
    let last_char: char = template[template.len()-1..].chars().nth(0).unwrap();
    
    // First, make the initial pairs
    for n in 0..template.len()-1 {
        let tmp = &template[n..n+2];
        counts.insert(tmp.to_string(),
                      counts
                      .get(tmp)
                      .get_or_insert(&0)
                      .to_owned() + 1);
    }

    // Then iterate...
    for _ in 0..40 {
        // First, collect all the pairs that are created
        // as the creation should happen in sync
        let mut new_counts = HashMap::<String, i64>::new();
        for (key, val) in counts.iter() {
            if rules.contains_key(key) {
                let new_char = rules.get(key); // What goes between

                let new_first = key
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_string() + new_char.unwrap();
                new_counts.insert(new_first.clone(),
                                  *new_counts
                                  .get(&new_first)
                                  .get_or_insert(&0) + val);

                let new_second = new_char
                    .unwrap()
                    .to_owned() +
                    &key.chars()
                    .nth(1)
                    .unwrap()
                    .to_string();
                new_counts.insert(new_second.clone(),
                                  *new_counts
                                  .get(&new_second)
                                  .get_or_insert(&0) + val);


            } else {
                // No rule to evolve this pair -> we just keep it intact.
                new_counts.insert(key.to_string(), *val);
                println!("!!! {:?}",key);
            }
        }
        counts = new_counts;
    }
    
    // Take the quantity of the most common element and
    // subtract the quantity of the least common element

    // We only look at the first character of each pair: ABCDEF -> AB, BC, CD, DE, EF
    let mut char_counts = HashMap::<char, i64>::new();
    for (key, val) in counts.iter() {
        let c = key.chars().nth(0).unwrap();
        char_counts.insert(c, *char_counts.get(&c).get_or_insert(&0) + val);
    }
    // And then... add 1 for the last character
    char_counts.insert(last_char, *char_counts.get(&last_char).get_or_insert(&0) + 1);
    
    println!("{:?}", char_counts);
    let vals: Vec<i64> = char_counts.into_values().collect();
    let result = vals.iter().max().unwrap() - vals.iter().min().unwrap(); 
    println!("The number is {:?}", result);
}
