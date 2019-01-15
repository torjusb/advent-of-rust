use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Cant read file");

    let frequency = get_final_frequency(&input);
    println!("Final frequency is: {}", frequency);

    let duplicate_frequency = get_first_dup(&input);
    println!("First duplicate frequency is: {}", duplicate_frequency)
}

fn get_first_dup(input: &str) -> i32 {
    let mut prev_frequencies = HashSet::new();
    let mut frequency = 0;

    loop {
        for line in input.lines() {
            let change: i32 = line.parse().expect("Expected a number");
            frequency += change;
            if prev_frequencies.contains(&frequency) {
                return frequency;
            }
            prev_frequencies.insert(frequency);
        }
    }
}

fn get_final_frequency(input: &str) -> i32 {
    let mut frequency = 0;
    for line in input.lines() {
        let change: i32 = line.parse().expect("Expected a number");
        frequency += change;
    }

    frequency
}
