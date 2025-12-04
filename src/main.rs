use std::fs;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("Hello, world!");

    let puzzle = fs::read_to_string("day1.txt").unwrap();

    let start = Instant::now();
    let res = day1::day1(puzzle);
    let end = Instant::now();

    println!("Day1 Result: {}", res);
    println!("Time: {:?}", end.duration_since(start));

    let puzzle = fs::read_to_string("day1.txt").unwrap();

    let start = Instant::now();
    let res = day1::day1_1(puzzle);
    let end = Instant::now();

    println!("Day1.1 Result: {}", res);
    println!("Time: {:?}", end.duration_since(start));

    let puzzle = fs::read_to_string("day2.txt").unwrap();

    let start = Instant::now();
    let res = day2::day2(puzzle, false);
    let end = Instant::now();

    println!("Day2 Result: {}", res);
    println!("Time: {:?}", end.duration_since(start));

    let puzzle = fs::read_to_string("day2.txt").unwrap();

    let start = Instant::now();
    let res = day2::day2(puzzle, true);
    let end = Instant::now();

    println!("Day2.1 Result: {}", res);
    println!("Time: {:?}", end.duration_since(start));

    let puzzle = fs::read_to_string("day3.txt").unwrap();

    let start = Instant::now();
    let res = day3::part1(puzzle);
    let end = Instant::now();

    println!("Day3 Result: {}", res);
    println!("Time: {:?}", end.duration_since(start));

    let puzzle = fs::read_to_string("day3.txt").unwrap();

    let start = Instant::now();
    let res = day3::part2(puzzle);
    let end = Instant::now();

    println!("Day3.1 Result: {}", res);
    println!("Time: {:?}", end.duration_since(start));

    let puzzle = fs::read_to_string("day4.txt").unwrap();

    let start = Instant::now();
    let res = day4::part1(&puzzle);
    let end = Instant::now();

    println!("Day4 Result: {}", res);
    println!("Time: {:?}", end.duration_since(start));

    let start = Instant::now();
    let res = day4::part2(&puzzle);
    let end = Instant::now();

    println!("Day4.1 Result: {}", res);
    println!("Time: {:?}", end.duration_since(start));
}
