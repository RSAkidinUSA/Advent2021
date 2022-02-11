#![crate_name = "diagnostic"]
use std::io::prelude::*;

// Depth increase counter
// This is my Rust implementation of Day 1 of the Advent of code
// This program takes a list of depths from stdin
// and outputs the number of times that value increases

fn main() -> Result <(), ()>{
    let stdin = std::io::stdin();
    // Figure out how long each binary string is
    let len = match stdin.read_line(&mut String::new()) {
        Err(e) => panic!("Error reading the first line {:?}", e),
        Ok(i) => i - 1,
    };
    println!("Binary string length: {}", len);
    // NOTE: bit_count is in reverse bit order for parsing reasons - e.g. MSB = bit_count[0]
    let mut bit_count: Vec<i32> = vec![0; len];
    let handle = stdin.lock();

    // Read in the data
    let lines = handle.lines();
    for line in lines {
        if line.is_err() {
            return Err(());
        }
        let val = line.unwrap();
        val.as_bytes().iter().enumerate().for_each(|(i, x)| {
            bit_count[i] = match *x as char{
                '1' => bit_count[i] + 1,
                '0' => bit_count[i] - 1,
                _ => {println!("Unrecognized Value");
                      0},
            }
        });
        // println!("{:?}: {}, {}", v, v[0], v[1]);
    }
    // Data processing
    let mut gamma: i32 = 0;
    for i in 0..=len - 1 {
        gamma = gamma << 1;
        if bit_count[i] > 0 {
            gamma |= 1;
        } else if bit_count[i] == 0 {
            println!("Error, equal number of 1's and 0's for bit {}", i);
        }
    }
    let epsilon = gamma ^ ((1 << len) - 1);

    println!("Gamma: {:0len$b}, Epsilon: {:0len$b}, Consumption: {}",
            gamma, epsilon, gamma * epsilon, len = len);

    Ok(())
}

