use std::fs;

mod day1;
mod day2;

fn main() {
    println!("Hello, world!");

    let input_day1 = fs::read_to_string("day1.txt").unwrap();

    let res_day1 = day1::day1(input_day1);
    println!("Day1 Result: {}", res_day1);

    let input_day1 = fs::read_to_string("day1.txt").unwrap();

    let res_day1_1 = day1::day1_1(input_day1);
    println!("Day1.1 Result: {}", res_day1_1);

    let input_day2 = fs::read_to_string("day2.txt").unwrap();
    let res_day2 = day2::day2(input_day2, false);
    println!("Day2 Result: {}", res_day2);

    let input_day2 = fs::read_to_string("day2.txt").unwrap();
    let res_day2 = day2::day2(input_day2, true);
    println!("Day2.1 Result: {}", res_day2);
}
