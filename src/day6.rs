struct Problem {
    op: char,
    res: u64,
}

impl Clone for Problem {
    fn clone(&self) -> Self {
        Problem {
            op: self.op,
            res: self.res,
        }
    }
}

pub fn part1(puzzle: &str) -> u64 {
    let mut sum = 0u64;

    let lines = puzzle.lines();

    let mut problems: Vec<Problem> = vec![];

    for (i, line) in lines.rev().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        if i == 0 {
            problems = vec![
                Problem {
                    op: '\0',
                    res: 0u64,
                };
                parts.len()
            ];

            for (j, part) in parts.iter().enumerate() {
                problems[j].op = part.chars().next().unwrap();
                if problems[j].op == '*' {
                    problems[j].res = 1;
                }
            }

            continue;
        }

        for (j, part) in parts.iter().enumerate() {
            let num = part.parse::<u64>().unwrap();

            match problems[j].op {
                '+' => problems[j].res += num,
                '*' => problems[j].res *= num,
                _ => continue,
            }
        }
    }

    for problem in problems {
        sum += problem.res;
    }

    sum
}

struct Problem2 {
    op: char,
    cols: usize,
}

pub fn part2(puzzle: &str) -> u64 {
    let mut sum = 0u64;

    // init problems
    let mut problems: Vec<Problem2> = vec![];

    let chars = puzzle.lines().rev().next().unwrap().chars();

    let mut last_char = chars.clone().next().unwrap();
    let mut cols = 1usize;

    for char in chars.skip(1) {
        if char != ' ' {
            problems.push(Problem2 {
                op: last_char,
                cols: cols - 1,
            });
            last_char = char;
            cols = 0;
        }

        cols += 1;
    }

    if cols > 0 {
        problems.push(Problem2 {
            op: last_char,
            cols,
        });
    }

    let num_count = puzzle.lines().count() - 1;

    // solve problems
    let mut pos = 0;
    for problem in problems {
        let mut res = 0u64;
        if problem.op == '*' {
            res = 1;
        }

        for i in pos..(pos + problem.cols) {
            let mut num = 0u64;

            for line in puzzle.lines().take(num_count) {
                let char = line.chars().nth(i).unwrap();
                if char == ' ' {
                    continue;
                }

                num = num * 10 + (char as u64 - '0' as u64);
            }

            match problem.op {
                '+' => res += num,
                '*' => res *= num,
                _ => continue,
            }
        }

        pos += problem.cols + 1;
        sum += res;
    }

    sum
}
