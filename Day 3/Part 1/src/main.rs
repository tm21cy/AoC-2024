use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should read input file");
    let re = Regex::new(r"mul\(\d*,\d*\)")
        .unwrap();
    let mut total: i64 = 0;
    for capture in re.captures_iter(&*input) {
        let extracted = &capture[0][4..&capture[0].len() - 1]
            .split(",")
            .collect::<Vec<&str>>();
        let instance_total: i64 = extracted[0]
            .parse::<i64>()
            .unwrap()
            * extracted[1]
            .parse::<i64>()
            .unwrap();
        total += instance_total;
    }
    println!("{}", total)
}
