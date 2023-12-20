use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    // let max_red = 12;
    // let max_blue = 14;
    // let max_green = 13;

    let mut qualified_game = vec![];
    let mut game_powers = vec![];

    for line in reader.lines() {
        match line {
            Ok(text) => {
                let groups: Vec<&str> = text.split(':').collect();
                let game = groups[0];
                let colors = groups[1];

                let mut red = 0;
                let mut blue = 0;
                let mut green = 0;

                let re = Regex::new(r"(\d+)\s+(\w+)").unwrap();
                for cap in re.captures_iter(colors) {
                    if let Some(number) = cap.get(1).map(|m| m.as_str().parse::<u32>().unwrap()) {
                        if let Some(color) = cap.get(2).map(|m| m.as_str()) {
                            match color {
                                "red" => red = cmp::max(red, number),
                                "blue" => blue = cmp::max(blue, number),
                                "green" => green = cmp::max(green, number),
                                _ => {} // Ignore other colors or handle them differently
                            }
                        }
                    }
                }

                dbg!("r {} g {} b {}", red, blue, green);
                dbg!("power {}", red * blue * green);

                // if red <= max_red && blue <= max_blue && green <= max_green {
                let re = Regex::new(r"Game (\d+)").unwrap();
                for cap in re.captures_iter(game) {
                    qualified_game.push(cap[1].parse::<u32>().unwrap());
                    game_powers.push(red * blue * green);
                }
                // }
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }
    let sum: u32 = qualified_game.iter().sum();

    println!("Qualified games: {:?}", qualified_game);
    println!("Qualified games: {:?}", sum);

    let power_sum: u32 = game_powers.iter().sum();
    println!("Powers games: {:?}", power_sum);

    Ok(())
}
