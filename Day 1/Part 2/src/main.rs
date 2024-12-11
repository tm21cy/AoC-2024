use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should read input file");
    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .enumerate()
        .for_each(|(idx, num)| {
            if idx % 2 == 0 {
                list_2.push(num)
            } else {
                list_1.push(num)
            }
        });
    list_1.sort();
    list_2.sort();

    let mut elements = 0;
    let mut sim: i64 = 0;
    while elements < list_1.len() {
        let sim_element: i32 = list_1[elements];
        let filtered: Vec<i32> = list_2
            .iter()
            .filter(|&&s| s == sim_element)
            .cloned()
            .collect();

        sim += (sim_element * filtered.len() as i32) as i64;
        elements += 1
    }
    println!("{}", sim)
}
