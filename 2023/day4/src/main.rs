use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut map = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(text) => {
                let groups: Vec<&str> = text.split(':').collect();
                let card = groups[0].split_whitespace().nth(1);

                let numbers: Vec<&str> = groups[1].split('|').collect();
                let pick = numbers[0]
                    .split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect::<HashSet<i32>>();

                let winning = numbers[1]
                    .split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect::<HashSet<i32>>();

                let count = winning.intersection(&pick).count();
                if let Some(card_no) = card {
                    if let Ok(parsed_card) = card_no.parse::<i32>() {
                        map.entry(parsed_card)
                            .and_modify(|x| *x += 1)
                            .or_insert(1_i64);
                        if let Some(&current_count) = map.get(&parsed_card) {
                            dbg!(
                                "card {} copy {} next count {}",
                                &parsed_card,
                                &current_count,
                                count
                            );
                            for c in 0..count {
                                map.entry(parsed_card + 1 + c as i32)
                                    .and_modify(|x| *x += current_count)
                                    .or_insert(current_count);
                            }
                            total += current_count;
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    dbg!("total {}", total);

    Ok(())
}

// // fn main() -> io::Result<()> {
// //     let file = File::open("input.txt")?;
// //     let reader = BufReader::new(file);

// //     let mut total = 0;

// //     for line in reader.lines() {
// //         match line {
// //             Ok(text) => {
// //                 let groups: Vec<&str> = text.split(':').collect();
// //                 let card = groups[0];

// //                 let numbers: Vec<&str> = groups[1].split('|').collect();
// //                 let pick: Vec<i32> = numbers[0]
// //                     .split_whitespace()
// //                     .filter_map(|x| x.parse().ok())
// //                     .collect();

// //                 let winning: Vec<i32> = numbers[1]
// //                     .split_whitespace()
// //                     .filter_map(|x| x.parse().ok())
// //                     .collect();

// //                 let count = pick.iter().filter(|&x| winning.contains(x)).count();
// //                 dbg!("count {}", count);
// //                 if count > 0 {
// //                     total += 2u32.pow(count as u32 - 1);
// //                 }
// //                 // let re = Regex::new(r"Game (\d+)").unwrap();
// //                 // for cap in re.captures_iter(game) {
// //                 //     qualified_game.push(cap[1].parse::<u32>().unwrap());
// //                 //     game_powers.push(red * blue * green);
// //                 // }
// //             }
// //             Err(err) => {
// //                 eprintln!("Error reading line: {}", err);
// //             }
// //         }
// //     }

// //     dbg!("total {}", total);

// //     Ok(())
// // }
