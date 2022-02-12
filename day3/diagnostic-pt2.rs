#![crate_name = "diagnostic"]
use std::io::prelude::*;

enum RatingType {
    O2,
    CO2,
}

fn get_rating(values: &Vec<u32>, len: usize, rt: &RatingType) -> Result<u32, &'static str> {
    let mut rating: u32 = 0;
    let mut filter: u32 = 0;
    let mut remaining = values.len();
    for i in 0..len {
        let mask = 1 << (len - i - 1);
        // # 1s >= number 0s
        let ones = values
            .iter()
            /* let's filter out any values which don't match the rating so far */
            .filter(|&x| ((x & filter) ^ rating) == 0)
            /* this will tell us how many 1's are in this position */
            .filter(|&x| x & mask != 0).count();
        /* We will check if the number of ones is less than number of zeros 
         * for this bit position, and we will act accordingly */
        /*
        println!("rating: {:0len$b}, remaining: {}, Mask: {:0len$b}, Ones: {}",
                rating, remaining, mask, ones, len=len);
        */
        let (remaining_tmp, rating_tmp) = match (remaining, rt, ones >= (remaining - ones)) {
            (1, _, true) => (1, rating | mask),
            (1, _, false) => (1, rating),
            (_, RatingType::O2, false) => (remaining - ones, rating),
            (_, RatingType::O2, true) => (ones, rating | mask),
            (_, RatingType::CO2, false) => (ones, rating | mask),
            (_, RatingType::CO2, true) => (remaining - ones, rating),
        };
        remaining = remaining_tmp;
        rating = rating_tmp;
        filter = filter | mask;
    };
    match remaining {
        1 => Ok(rating),
        0 => Err("No values left"),
        _ => Err("More than one value"),
    }
}

// Depth increase counter
// This is my Rust implementation of Day 1 of the Advent of code
// This program takes a list of depths from stdin
// and outputs the number of times that value increases

fn main() -> Result <(), ()>{
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut readings = vec![];

    // Read in the data
    let mut len = 0;
    let lines = handle.lines();
    for line in lines {
        let data = match line {
            Ok(data) => data,
            Err(e) => panic!("Error reading line {:?}", e),
        };
        len = match len >= data.len() {
            true => len,
            false => data.len(),
        };
        let val = u32::from_str_radix(&data, 2).unwrap();
        readings.push(val);
    };
    // Data processing
    // println!("Getting O2 Rating");
    let o2 = match get_rating(&readings, len, &RatingType::O2) {
        Err(e) => panic!("Error with O2: {:?}", e),
        Ok(i) => i,
    };
    // println!("Getting CO2 Rating");
    let co2 = match get_rating(&readings, len, &RatingType::CO2) {
        Err(e) => panic!("Error with CO2: {:?}", e),
        Ok(i) => i,
    };

    println!("O2 Generator Rating: {:0len$b}, CO2 Scubber Rating: {:0len$b}, \
            Life Support Rating: {}",
            o2, co2, o2 * co2, len = len);

    Ok(())
}
