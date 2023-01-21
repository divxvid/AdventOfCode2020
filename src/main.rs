mod day2;

use std::{fs, time::SystemTime};

const FILE_PATH: &str = "./data/input02.txt";

fn timeit<F: Fn() -> T, T>(f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Time Taken: {} ms.", duration.as_millis());
    result
}

fn main() {
    let input_data = fs::read_to_string(FILE_PATH).expect("File Not Found");
    println!("Part one output: {}", timeit(|| day2::part_one(&input_data)));
    println!("Part two output: {}", timeit(|| day2::part_two(&input_data)));
}
