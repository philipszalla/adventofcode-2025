pub fn part1(puzzle: &str) -> u64 {
    let mut sum = 0;

    let mut width: i32 = 0;
    let mut height: i32 = 0;

    let mut diagram: Vec<char> = Vec::new();
    for line in puzzle.split('\n') {
        if width == 0 {
            width = line.len() as i32;
        }
        height += 1;

        for char in line.chars() {
            diagram.push(char);
        }
    }

    let size = width * height;

    for pos in 0..size {
        if diagram[pos as usize] != '@' {
            continue;
        }

        if is_accessible(&diagram, pos % width, pos / height, width, height) {
            sum += 1;
        }
    }

    sum
}

pub fn part2(puzzle: &str) -> u64 {
    let mut sum = 0;

    let mut width: i32 = 0;
    let mut height: i32 = 0;

    let mut diagram: Vec<char> = Vec::new();
    for line in puzzle.split('\n') {
        if width == 0 {
            width = line.len() as i32;
        }
        height += 1;

        for char in line.chars() {
            diagram.push(char);
        }
    }

    let size = width * height;

    loop {
        let mut changed = false;

        for pos in 0..size {
            if diagram[pos as usize] != '@' {
                continue;
            }

            if is_accessible(&diagram, pos % width, pos / height, width, height) {
                sum += 1;
                changed = true;

                diagram[pos as usize] = '.';
            }
        }

        if !changed {
            break;
        }
    }

    sum
}

fn is_accessible(diagram: &Vec<char>, x: i32, y: i32, width: i32, height: i32) -> bool {
    let mut adjacents = 0;

    for x2 in (x - 1)..=(x + 1) {
        for y2 in (y - 1)..=(y + 1) {
            if x2 < 0 || x2 >= width || y2 < 0 || y2 >= height {
                // out of grid
                continue;
            }

            if x == x2 && y == y2 {
                // current position
                continue;
            }

            let pos = y2 * height + x2;

            if diagram[pos as usize] == '@' {
                adjacents += 1;
            }
        }
    }

    adjacents < 4
}
