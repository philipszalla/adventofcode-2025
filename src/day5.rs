pub fn part1(puzzle: &str) -> u64 {
    let mut sum = 0u64;

    let mut ranges: Vec<(u64, u64)> = vec![];

    let mut init = true;
    for line in puzzle.lines() {
        if line == "" {
            init = false;

            continue;
        }

        if init {
            let parts = line.split("-").collect::<Vec<&str>>();
            let min = parts[0].parse::<u64>().unwrap();
            let max = parts[1].parse::<u64>().unwrap();
            ranges.push((min, max));

            continue;
        }

        let ingredient = line.parse::<u64>().unwrap();
        for range in &ranges {
            if range.0 <= ingredient && ingredient <= range.1 {
                sum += 1;
                break;
            }
        }
    }

    sum
}

pub fn part2(puzzle: &str) -> u64 {
    let mut sum = 0u64;

    let mut all_ranges: Vec<(u64, u64)> = vec![];

    for line in puzzle.lines() {
        if line == "" {
            break;
        }

        let parts = line.split("-").collect::<Vec<&str>>();
        let min = parts[0].parse::<u64>().unwrap();
        let max = parts[1].parse::<u64>().unwrap();

        all_ranges.push((min, max));
    }

    all_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_ranges: Vec<(u64, u64)> = vec![];

    let mut min = all_ranges[0].0;
    let mut max = all_ranges[0].1;

    for range in &all_ranges {
        if range.0 > max {
            merged_ranges.push((min, max));

            min = range.0;
            max = range.1;
            continue;
        }

        if range.1 > max {
            max = range.1;
        }
    }

    merged_ranges.push((min, max));

    for range in &merged_ranges {
        sum += (range.1 - range.0) + 1;
    }

    sum
}
