use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the file in read-only mode
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);
    let mut total = 0;

    let mut word_to_digit = HashMap::new();
    // Populate the HashMap with mappings
    word_to_digit.insert("one", 1);
    word_to_digit.insert("two", 2);
    word_to_digit.insert("three", 3);
    word_to_digit.insert("four", 4);
    word_to_digit.insert("five", 5);
    word_to_digit.insert("six", 6);
    word_to_digit.insert("seven", 7);
    word_to_digit.insert("eight", 8);
    word_to_digit.insert("nine", 9);
    word_to_digit.insert("1", 1);
    word_to_digit.insert("2", 2);
    word_to_digit.insert("3", 3);
    word_to_digit.insert("4", 4);
    word_to_digit.insert("5", 5);
    word_to_digit.insert("6", 6);
    word_to_digit.insert("7", 7);
    word_to_digit.insert("8", 8);
    word_to_digit.insert("9", 9);
    word_to_digit.insert("0", 0);

    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(text) => {
                let mut temp = vec![];
                let mut left = 0;
                let mut right = 0;
                while left < text.len() {
                    let word = text.get(left..=right).unwrap();
                    if word_to_digit.contains_key(word) {
                        temp.push(*word_to_digit.get(word).unwrap());
                        left += 1;
                        right = left;
                        continue;
                    }
                    if right - left == 4 || right == text.len() - 1 {
                        left += 1;
                        right = left;
                        continue;
                    }

                    right += 1;
                }
                dbg!("{}", temp.clone());
                let num = temp.first().unwrap() * 10 + temp.last().unwrap();

                total += num;
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    println!("{}", total);
    Ok(())
}
