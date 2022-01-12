#![crate_name = "depth"]
use std::io::prelude::*;

// Depth increase counter
// This is my Rust implementation of Day 1 of the Advent of code
// This program takes a list of depths from stdin
// and outputs the number of times that value increases

fn main() -> std::io::Result <()>{
    let mut last_depth: i32 = 0;
    let mut count_inc = 0;
    let mut first_pass = true;
    let mut curr_depth: i32;
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        curr_depth = line?.parse().unwrap();
                
        count_inc =
            if !first_pass && curr_depth > last_depth {
                count_inc + 1
            } else {
                count_inc
            };
        first_pass = false;
        last_depth = curr_depth;
    }

    println!("{} Measurements larger than the previous measurement", count_inc);
    Ok(())
}
