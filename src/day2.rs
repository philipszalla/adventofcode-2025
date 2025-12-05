pub fn part1(puzzle: &str) -> u64 {
    solve(puzzle, false)
}

pub fn part2(puzzle: &str) -> u64 {
    solve(puzzle, true)
}

fn solve(puzzle: &str, part2: bool) -> u64 {
    let mut sum = 0u64;

    let mut invalid_fn: fn(u64) -> bool = invalid_id;
    if part2 {
        invalid_fn = invalid_id_part2;
    }

    let ranges = puzzle.split(",");
    for range in ranges {
        let parts = range.split("-");
        let start = parts.clone().nth(0).unwrap().parse::<u64>().unwrap();
        let end = parts.clone().nth(1).unwrap().parse::<u64>().unwrap();

        for num in start..=end {
            if invalid_fn(num) {
                sum += num;
            }
        }
    }

    sum
}

fn invalid_id(num: u64) -> bool {
    let len = num_len(&num);

    if len % 2 != 0 {
        // No pattern possible
        return false;
    }

    let pattern_len = len / 2;

    let last = sub_int(&num, pattern_len, 0);
    let next = sub_int(&num, pattern_len, 1);

    if next == last {
        return true;
    }

    false
}

fn invalid_id_part2(num: u64) -> bool {
    let len = num_len(&num);

    if len < 2 {
        // No pattern possible
        return false;
    }

    for pattern_len in 1..=(len / 2) {
        let mut invalid = true;

        if len % pattern_len != 0 {
            // No equal pattern possible
            continue;
        }

        let last = sub_int(&num, pattern_len, 0);
        for pattern_index in 1..(len / pattern_len) {
            let next = sub_int(&num, pattern_len, pattern_index);

            if next != last {
                invalid = false;
                break;
            }
        }

        if invalid {
            return true;
        }
    }

    false
}

fn num_len(num: &u64) -> i32 {
    let mut copy = *num;
    let mut len = 1;

    while copy > 9 {
        copy /= 10;
        len += 1;
    }

    len
}

fn sub_int(num: &u64, len: i32, index: i32) -> u64 {
    let moved_num = num / 10u64.pow((index * len) as u32);

    if num_len(&moved_num) == len {
        return moved_num;
    }

    // moved_num - moved_num / 10i64.pow(len as u32) * 10i64.pow(len as u32)
    moved_num % 10u64.pow(len as u32)
}
