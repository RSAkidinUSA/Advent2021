#![crate_name = "navigate"]
use std::io::prelude::*;

// Depth increase counter
// This is my Rust implementation of Day 1 of the Advent of code
// This program takes a list of depths from stdin
// and outputs the number of times that value increases

fn main() -> std::io::Result <()>{
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let data = line?;
        let mut mag: i32 = 0;
        let v: Vec<&str> = data.split_whitespace().collect();
        match v[0] {
            "forward" => mag += v[1].parse::<i32>().unwrap(),
            "down" => aim += v[1].parse::<i32>().unwrap(),
            "up" => aim -= v[1].parse::<i32>().unwrap(),
            _ => println!("Unrecognized command"),
        };
        pos += mag;
        depth += aim * mag;
        // println!("{:?}: {}, {}", v, v[0], v[1]);
    }
    println!("Depth: {}, Position: {}, Mult: {}", depth, pos, depth * pos);

    Ok(())
}
