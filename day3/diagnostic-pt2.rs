#![crate_name = "diagnostic"]
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::cmp::Ordering;

enum RatingType {
    O2,
    CO2,
}

fn get_rating(v: &Vec<u32>, len: usize, rt: &RatingType) -> std::io::Result<u32> {
    let mut values = v.clone();
    for i in 0..len {
        let mask = 1 << (len - i - 1);
        // # 1s >= number 0s
        let (ones, zeros): (Vec<u32>, Vec<u32>) = values
            .iter()
            .partition(|&x| x & mask != 0);     
        values = match (rt, ones.len().cmp(&zeros.len())) {
            (RatingType::O2, Ordering::Less) => zeros,
            (RatingType::O2, _) => ones,
            (RatingType::CO2, Ordering::Less) => ones,
            (RatingType::CO2, _) => zeros,
        };
        if values.len() <= 1 {
            break;
        }
    };
    match values.len() {
        1 => Ok(values.remove(0)),
        0 => Err(Error::new(ErrorKind::Other, "No values left")),
        _ => Err(Error::new(ErrorKind::Other, "More than one value")),
    }
}

// Depth increase counter
// This is my Rust implementation of Day 1 of the Advent of code
// This program takes a list of depths from stdin
// and outputs the number of times that value increases

fn main() -> std::io::Result <()>{
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut readings = vec![];

    // Read in the data
    let mut len = 0;
    for line in handle.lines() {
        let data = line?;
        len = match len >= data.len() {
            true => len,
            false => data.len(),
        };
        let val = u32::from_str_radix(&data, 2).unwrap();
        readings.push(val);
    };
    // Data processing
    let o2 = get_rating(&readings, len, &RatingType::O2)?;
    let co2 = get_rating(&readings, len, &RatingType::CO2)?;

    println!("O2 Generator Rating: {:0len$b}, CO2 Scubber Rating: {:0len$b}, Life Support Rating: {}",
            o2, co2, o2 * co2, len = len);

    Ok(())
}
