mod day1;
mod day2;
mod day3;
mod day4;
mod util;

fn main() {
    util::run_day(1, day1::part1, day1::part2);
    util::run_day(2, day2::part1, day2::part2);
    util::run_day(3, day3::part1, day3::part2);
    util::run_day(4, day4::part1, day4::part2);
}
