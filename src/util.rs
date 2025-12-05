use std::fs;
use std::time::Instant;

pub fn run_day(day: u8, part1: fn(&str) -> u64, part2: fn(&str) -> u64) {
    let puzzle = read_file(day);
    
    let start = Instant::now();
    let res = part1(&puzzle);
    let end = Instant::now();

    println!("Day {} Part 1 Result: {}", day, res);
    println!("Time: {:?}", end.duration_since(start));

    let start = Instant::now();
    let res = part2(&puzzle);
    let end = Instant::now();

    println!("Day {} Part 2 Result: {}", day, res);
    println!("Time: {:?}", end.duration_since(start));
}

fn read_file(day: u8) -> String {
    let file_name = format!("day{}.txt", day);

    fs::read_to_string(&file_name).expect(&(file_name + " is not a file"))
}
