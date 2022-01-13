#![crate_name = "depth"]
use std::io::prelude::*;

// Depth increase counter
// This is my Rust implementation of Day 1 of the Advent of code
// This program takes a list of depths from stdin
// and outputs the number of times that value increases

#[derive(Debug)]
struct Window {
    sum: i32,
}

fn main() -> std::io::Result <()>{
    let mut count_inc = 0;
    let mut num_passes = 0;
    let mut curr_depth: i32;
    let mut prev = Window {sum: 0};
    let mut curr = Window {sum: 0};
    let mut next = Window {sum: 0};
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        curr_depth = line?.parse().unwrap();
        
        if num_passes <= 2 {
            prev.sum += curr_depth;
        }
        if num_passes > 0 {
            curr.sum += curr_depth;
        }
        if num_passes > 1 {
            next.sum += curr_depth;
        }
        if num_passes == 3 {
            println!("{}, N/A", prev.sum);
        }
        if num_passes > 2 {
            count_inc =
                if curr.sum > prev.sum {
                    count_inc + 1
                } else {
                    count_inc
                };
            println!("{}, {}", curr.sum, curr.sum > prev.sum);
            prev.sum = curr.sum;
            curr.sum = next.sum;
            next.sum = curr_depth;
        }
        num_passes += 1;
    }

    println!("{} Measurements larger than the previous measurement", count_inc);
    Ok(())
}
