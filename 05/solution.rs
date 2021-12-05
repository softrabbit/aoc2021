// https://adventofcode.com/2021/day/5

// Part 1: Check for intersecting horizontal and vertical lines
// Part 2: Consider also diagonal lines (Compile with rustc --cfg part2 )

use std::io::{self, BufRead};
use std::cmp;

// Returns the numbers from a string like '0,1 -> 2,3'
// This might be better, but I don't feel like getting into Cargo today:
// let re = Regex::new("([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").unwrap();
fn get_numbers(in_str: &str) -> [i32; 4] {
    let parts = in_str.split(" -> ");
    let mut n=0;
    let mut numbers: [i32; 4] = [0,0,0,0];
    for part in parts {
        let num_parts: Vec<&str> = part.split(',').collect();
        numbers[n] = num_parts[0].parse::<i32>().unwrap();
        numbers[n+1] = num_parts[1].parse::<i32>().unwrap();
        n += 2;
    }
    numbers
}

fn main() {
    let stdin = io::stdin();
    let mut input = Vec::new();
    
    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }
    
    let mut map = vec![vec![0; 1]; 1]; // 1x1, this will grow later
    for line in input {
        let [x1, y1, x2, y2] = get_numbers(&line);

        // Map growing logic...
        let mut w = map[0].len();
        let h = map.len();
        let x_max = (cmp::max(x1, x2)+1) as usize;
        let y_max = (cmp::max(y1, y2)+1) as usize;
        if x_max > w {
            // Resize every row
            for row in map.iter_mut() {
                row.resize(x_max,0);
            }
            w = x_max;            
        }
        if y_max > h {
            // Add rows at end
            map.resize(y_max, vec![0; w]);
        }

        // Then, make the line (horizontal or vertical, skip others)
        if x1==x2 {
            let (from, to) = if y1 > y2 { (y2, y1) } else { (y1,y2) };
            for y in from..=to {
                map[y as usize][x1 as usize] += 1;
            }
        } else if y1 == y2 {
            let (from, to) = if x1 > x2 { (x2, x1) } else { (x1,x2) };
            for x in from..=to {
                map[y1 as usize][x as usize] += 1;
            }
        } else { // Diagonal lines, always 45 degrees
            #[cfg(part2)] {
                // Turn the limits around...
                let (from, to, xfrom, xto) = if y1 > y2 {
                    (y2, y1, x2, x1)
                } else {
                    (y1, y2, x1, x2)
                };
                let x_inc = if xfrom > xto { -1 } else { 1 };
                let mut x = xfrom;
                for y in from..=to {
                    map[y as usize][x as usize] += 1;
                    x += x_inc;
                }
            }
        }
        
    }
    // Last, count the number of intersections ( map[x][y] >= 2)
    let mut n = 0; 
    for row in map {
        n += row.into_iter().filter(|i| i>= &2).count();
    }

    println!("The number is {:?}", n);
}
