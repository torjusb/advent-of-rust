use std::collections::HashMap;

fn calculate_score(input: &str) -> u32 {
    let mut letters_of_twos = 0;
    let mut letters_of_threes = 0;

    for line in input.lines() {
        let mut letter_scores = HashMap::new();
        for letter in line.chars() {
            let count = letter_scores.entry(letter).or_insert(0);
            *count += 1;
        }

        let mut found_two = false;
        let mut found_three = false;
        for (_letter, score) in &letter_scores {
            if score == &2 && !found_two {
                letters_of_twos += 1;
                found_two = true;
            } else if score == &3 && !found_three {
                letters_of_threes += 1;
                found_three = true;
            }
        }
    }

    letters_of_twos * letters_of_threes
}

fn differs_by_one(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut num_diffs = 0;
    for i in 0..a.len() {
        if a.chars().nth(i).unwrap() != b.chars().nth(i).unwrap() {
            num_diffs += 1;
        }
        if num_diffs > 2 {
            break;
        }
    }

    num_diffs == 1
}

fn get_similar_chars(a: &str, b: &str) -> String {
    let mut b = b.chars();
    let result: String = a
        .chars()
        .filter(|char| *char == b.nth(0).unwrap())
        .collect();
    result
}

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("Can't read file");

    println!("Result part 1: {}", calculate_score(&input));

    for line in input.lines() {
        for line2 in input.lines() {
            if differs_by_one(&line, &line2) {
                println!("Result part 2: {}", get_similar_chars(line, line2));
                return;
            }
        }
    }
}
