pub fn part1(puzzle: &str) -> u64 {
    let mut sum = 0u64;

    for line in puzzle.lines() {
        let mut first_char = '0';
        let mut index: usize = 0;

        for (i, c) in line.chars().enumerate() {
            if i + 1 == line.len() {
                break;
            }

            if c > first_char {
                first_char = c;
                index = i;
            }

            if first_char == '9' {
                break;
            }
        }

        let mut joltage = (first_char as u64 - '0' as u64) * 10;

        let mut last_char = '0';
        for (i, c) in line.chars().enumerate() {
            if i <= index {
                continue;
            }

            if c > last_char {
                last_char = c;
                index = i;
            }

            if last_char == '9' {
                break;
            }
        }

        joltage += last_char as u64 - '0' as u64;

        sum += joltage;
    }

    sum
}

pub fn part2(puzzle: &str) -> u64 {
    let mut sum = 0;

    for line in puzzle.lines() {
        let mut joltage = 0u64;
        let mut next_index: usize = 0;

        for pos in 1..=12 {
            let remaining = 12 - pos;

            let mut char = '0';
            let mut index: usize = 0;

            for (i, c) in line.chars().enumerate() {
                if i < next_index {
                    continue;
                }

                if i + remaining >= line.len() {
                    break;
                }

                if c > char {
                    char = c;
                    index = i;
                }

                if c == '9' {
                    break;
                }
            }

            joltage = joltage * 10 + (char as u64 - '0' as u64);
            next_index = index + 1;
        }

        sum += joltage;
    }

    sum
}
