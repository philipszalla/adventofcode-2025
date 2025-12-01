use std::fs;

mod day1;

fn main() {
    println!("Hello, world!");

    let input_day1 = fs::read_to_string("day1.txt").unwrap();

    let res_day1 = day1::day1(input_day1);
    println!("Day1 Result: {}", res_day1);

    let input_day1 = fs::read_to_string("day1.txt").unwrap();

    let res_day1_1 = day1::day1_1(input_day1);
    println!("Day1.1 Result: {}", res_day1_1);
}
