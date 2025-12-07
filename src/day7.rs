pub fn part1(puzzle: &str) -> u64 {
    let mut sum = 0u64;

    let cols = puzzle.lines().next().unwrap().len();
    let mut beams = vec![false; cols];

    // find start
    for (i, char) in puzzle.lines().next().unwrap().chars().enumerate() {
        if char == 'S' {
            beams[i] = true;
            break;
        }
    }

    for line in puzzle.lines().skip(1) {
        for (i, char) in line.chars().enumerate() {
            if char == '^' && beams[i] {
                beams[i] = false;
                if i > 0 {
                    beams[i - 1] = true;
                }
                if i < cols - 1 {
                    beams[i + 1] = true;
                }
                sum += 1;
            }
        }
    }

    sum
}

pub fn part2(puzzle: &str) -> u64 {
    let rows = puzzle.lines().count() - 1;
    let cols = puzzle.lines().next().unwrap().len();

    let cleared = puzzle.replace('\n', "");
    let cleared_puzzle = cleared.get(cols..).unwrap();

    let mut cache = vec![-1i64; rows * cols];

    // find start
    for (i, char) in puzzle.lines().next().unwrap().chars().enumerate() {
        if char == 'S' {
            return traverse(cleared_puzzle, &rows, &cols, cache.as_mut_slice(), 0, i);
        }
    }

    0
}

fn traverse(
    puzzle: &str,
    rows: &usize,
    cols: &usize,
    cache: &mut [i64],
    row: usize,
    col: usize,
) -> u64 {
    if row >= *rows {
        return 1;
    }

    let cell = row * rows + col;
    if cache[cell] != -1 {
        return cache[cell] as u64;
    }

    let mut sum = 0u64;

    let char = puzzle.chars().nth(cell).unwrap();
    if char == '.' {
        sum = traverse(puzzle, rows, cols, cache, row + 1, col);
    } else {
        if col > 0 {
            sum += traverse(puzzle, rows, cols, cache, row + 1, col - 1)
        }
        if col < rows - 1 {
            sum += traverse(puzzle, rows, cols, cache, row + 1, col + 1)
        }
    }

    cache[cell] = sum as i64;

    sum
}
