use std::fs;
use std::ops::Index;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should read input file");
    let puzzle: Vec<&str> = input.lines().collect();
    let num_puzzles = puzzle.len();

    let mut unsafe_reports: i32 = 0;
    for x in puzzle {
        println!("Puzzle initiated...");
        println!("{:?}", x);
        let puzzle_ints: Vec<i32> = x.split(" ")
            .map(|input| input.parse::<i32>().unwrap())
            .collect();
        let mut last_int: Option<i32> = None;
        let mut last_increasing: Option<bool> = None;
        for int in puzzle_ints {
            // First, check to see if the last_int has been set. If not, we're at the first index.
            match last_int {
                None => {
                    last_int = Some(int);
                    continue
                }
                // If it is set, we continue.
                Some(last) => {
                    // Next, check the direction - if it hasn't been established, establish it.
                    match last_increasing {
                        None => {
                            last_increasing = Some(int > last);
                        }
                        Some(increasing) => {}
                    }
                    // Check requisite conditions:
                    // 1. track if increasing when it should be decreasing, vice versa
                    // 2. track gap to ensure 1 <= gap <= 3
                    let increasing = last_increasing.unwrap();
                    let is_ints_increasing = int > last;
                    let gap = i32::abs(int - last) <= 3 && i32::abs(int - last) >= 1;

                    // check both directional conditions
                    if increasing && is_ints_increasing && gap {
                        last_int = Some(int);
                        continue
                    } else if !increasing && !is_ints_increasing && gap {
                        last_int = Some(int);
                        continue
                    } else {
                        unsafe_reports += 1;
                        break
                    }
                }
            }
        }
    }
    println!("{}", num_puzzles as i32 - unsafe_reports)
}
