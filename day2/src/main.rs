use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()).unwrap_or(String::from("A")),)*)
    }}
}

trait Compare {
    fn compare(&self, opp: &str) -> i32;
}

impl Compare for str {
    fn compare(&self, opp: &str) -> i32 {
        match (self, opp) {
            // A | X == Rock
            // B | Y == Paper
            // C | Z == Scissors
            ("X", "A") => 4,
            ("X", "B") => 1,
            ("X", "C") => 7,
            ("Y", "A") => 8,
            ("Y", "B") => 5,
            ("Y", "C") => 2,
            ("Z", "A") => 3,
            ("Z", "B") => 9,
            ("Z", "C") => 6,
            _ => 0,
        }
    }
}

fn calculate_points(filename: &str) -> i32 {
    let mut total_points: i32 = 0;
    let file: File = File::open(filename).expect("no such file");
    let buf: BufReader<File> = BufReader::new(file);
    let res: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).collect();
    for line in res {
        let (opp, me) = scan!(line, char::is_whitespace, String, String);
        total_points += me.as_str().compare(opp.as_str());
    }
    total_points
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default: &String = &String::from("input.txt");
    let filename: &str = args.get(1).unwrap_or(default);

    println!("{}", calculate_points(filename));
}
