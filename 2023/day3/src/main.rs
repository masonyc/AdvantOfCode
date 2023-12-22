use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        match line {
            Ok(text) => {
                matrix.push(text.chars().collect());
            }

            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    let mut visited = HashSet::new();
    let mut gear_ratios: Vec<i64> = vec![];
    let mut adjacent_numbers = vec![];

    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if let '*' = col {
                // Check adjacent cells for numbers
                for i in -1..=1 {
                    for j in -1..=1 {
                        let x = row_index as i32 + i;
                        let y = col_index as i32 + j;

                        // Ensure x and y are within bounds
                        if x >= 0
                            && x < matrix.len() as i32
                            && y >= 0
                            && y < row.len() as i32
                            && !(i == 0 && j == 0)
                        {
                            let adjacent_char = matrix[x as usize][y as usize];

                            if is_digit(adjacent_char) && !visited.contains(&(x, y)) {
                                // If the adjacent character is a digit, gather the full number
                                let mut num = String::new();
                                let mut k: i32 = -1;
                                                        num.push(adjacent_char);
                                while (y + k) >= 0 && (y + k) < matrix.len() as i32 {
                                    if !is_digit(matrix[x as usize][(y + k) as usize]) {
                                        break;
                                    }
                                    let cur_x = x as usize;
                                    let cur_y = (y + k) as usize;
                               
                                    num.push(matrix[cur_x][cur_y]);
                                    visited.insert((cur_x as i32, cur_y as i32));
                                    k -= 1;
                                }
                                num = num.chars().rev().collect();
                                k = 1;
                                while y + k >= 0 && y + k < matrix.len() as i32 {
                                    if !is_digit(matrix[x as usize][(y + k) as usize]) {
                                        break;
                                    }
                                    let cur_x = x as usize;
                                    let cur_y = (y + k) as usize;
                                    num.push(matrix[cur_x][cur_y]);
                                    visited.insert((cur_x as i32, cur_y as i32));
                                    k += 1;
                                }
                                dbg!("num {} x {} y {}", &num);
                                adjacent_numbers.push(num.parse::<i64>().unwrap());
                            }
                        }
                    }
                }

                if adjacent_numbers.len() >= 2 {
                    gear_ratios.push(adjacent_numbers.iter().product());
                }
                adjacent_numbers.clear();
            }
        }
    }

    dbg!("gear  {}", &gear_ratios);
    let sum:i64 = gear_ratios.iter().sum();
    dbg!("gear total {}", sum);

    Ok(())
}
// fn main() -> io::Result<()> {
//     let file = File::open("input.txt")?;
//     let reader = BufReader::new(file);

//     let mut matrix: Vec<Vec<char>> = vec![];
//     let mut adjacent_numbers = vec![];
//     for line in reader.lines() {
//         match line {
//             Ok(text) => {
//                 matrix.push(text.chars().collect());
//             }

//             Err(err) => {
//                 eprintln!("Error reading line: {}", err);
//             }
//         }
//     }

//     fn is_digit(ch: char) -> bool {
//         ch.is_ascii_digit()
//     }

//     let mut visited = HashSet::new();

//     for (row_index, row) in matrix.iter().enumerate() {
//         for (col_index, col) in row.iter().enumerate() {
//             match col {
//                 '*' | '+' | '$' | '#' | '-' | '@' | '&' | '%' | '/' | '=' => {
//                     // Check adjacent cells for numbers
//                     for i in -1..=1 {
//                         for j in -1..=1 {
//                             let x = row_index as i32 + i;
//                             let y = col_index as i32 + j;

//                             // Ensure x and y are within bounds
//                             if x >= 0
//                                 && x < matrix.len() as i32
//                                 && y >= 0
//                                 && y < row.len() as i32
//                                 && !(i == 0 && j == 0)
//                             {
//                                 let adjacent_char = matrix[x as usize][y as usize];

//                                 if is_digit(adjacent_char) && !visited.contains(&(x, y)) {
//                                     // If the adjacent character is a digit, gather the full number
//                                     let mut num = String::new();
//                                     let mut k: i32 = -1;
//                                     dbg!(
//                                         "adj char {} x {} k {} y{} x+k {}",
//                                         &adjacent_char,
//                                         &x,
//                                         &k,
//                                         &y,
//                                         x + k
//                                     );
//                                     dbg!("{} {}", y + k >= 0, (y + k) < matrix.len() as i32);

//                                     num.push(adjacent_char);
//                                     while (y + k) >= 0 && (y + k) < matrix.len() as i32 {
//                                         if !is_digit(matrix[x as usize][(y + k) as usize]) {
//                                             break;
//                                         }
//                                         let cur_x = x as usize;
//                                         let cur_y = (y + k) as usize;
//                                         dbg!(
//                                             "Going negtive {} cur_x {} cur_y {}",
//                                             &matrix[cur_x][cur_y],
//                                             cur_y,
//                                             cur_y
//                                         );
//                                         num.push(matrix[cur_x][cur_y]);
//                                         visited.insert((cur_x as i32, cur_y as i32));
//                                         k -= 1;
//                                     }
//                                     num = num.chars().rev().collect();
//                                     k = 1;
//                                     while y + k >= 0 && y + k < matrix.len() as i32 {
//                                         if !is_digit(matrix[x as usize][(y + k) as usize]) {
//                                             break;
//                                         }
//                                         let cur_x = x as usize;
//                                         let cur_y = (y + k) as usize;
//                                         dbg!(
//                                             "Going pos {} cur_x {} cur_y {}",
//                                             &matrix[cur_x][cur_y],
//                                             cur_y,
//                                             cur_y
//                                         );
//                                         num.push(matrix[cur_x][cur_y]);
//                                         visited.insert((cur_x as i32, cur_y as i32));
//                                         k += 1;
//                                     }
//                                     dbg!("num {} x {} y {}", &num);
//                                     adjacent_numbers.push(num.parse::<u32>().unwrap());
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }

//     dbg!("adj {}", &adjacent_numbers);
//     let sum: u32 = adjacent_numbers.iter().sum();
//     dbg!("adj total {}", sum);

//     Ok(())
// }
