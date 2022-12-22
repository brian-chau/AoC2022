use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

struct Elf {
    points: i32,
}

fn read_file(filename: &str) -> Vec<String> {
    let file: File = File::open(filename).expect("no such file");
    let buf: BufReader<File> = BufReader::new(file);
    buf.lines().map(|line| line.expect("Could not parse line")).collect()
}

fn determine_points(contents: &mut [String], count: usize) {
    let mut elves: Vec<Elf> = Vec::new();
    let mut elf: Elf = Elf { points: 0 };
    for line in contents {
        if line.is_empty() {
            elves.push(elf);
            elf = Elf { points: 0 };
        } else {
            elf.points += line.parse::<i32>().unwrap();
        }
    }
    elves.push(elf);
    elves.sort_by_key(|elf| elf.points);
    let max: i32 = elves.iter().rev().take(count).map(|elf| elf.points).sum();
    println!("Max: {}", max);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default: &String = &String::from("input.txt");
    let filename: &str = args.get(1).unwrap_or(default);

    let mut contents: Vec<String> = read_file(filename);
    determine_points(&mut contents, 1);
    determine_points(&mut contents, 3);
}
