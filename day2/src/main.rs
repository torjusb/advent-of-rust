use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("Can't read file");

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

    println!("Result: {}", letters_of_twos * letters_of_threes);

    for line in input.lines() {
        for line2 in input.lines() {
            let mut num_diffs = 0;
            for i in 0..line2.len() {
                if line.chars().nth(i) != line2.chars().nth(i) {
                    num_diffs += 1;
                }
                if num_diffs > 1 {
                    break;
                }
                if num_diffs == 1 {
                    println!("{}", line);
                }
            }
        }
    }
}
