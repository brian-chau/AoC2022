use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let default: &String = &String::from("input.txt");
    let filename: &str = args.get(1).unwrap_or(default);

    let entire_file: String = fs::read_to_string(filename).expect("Unable to read file");
    let new_data: &str = &entire_file[0..&entire_file.len() - 1];
    let calories_per_elf: Vec<Vec<i32>> = new_data.split("\n\n").map(|s| s.split('\n').map(|num| num.parse::<i32>().unwrap_or_else(|v| panic!("Num is invalid: {}", v))).collect()).collect();

    let mut sums: Vec<i32> = calories_per_elf.iter().map(|calories| calories.iter().sum::<i32>()).collect();

    sums.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", sums[0]);
    println!("Part 2: {:?}", sums[0] + sums[1] + sums[2]);
}
