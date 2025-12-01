pub fn day1(puzzle: String) -> i32 {
    let mut sum = 0;
    let mut pos = 50;

    for line in puzzle.lines() {
        let number = line.get(1..).unwrap();
        let mut delta = number.parse::<i32>().unwrap();

        if line.chars().nth(0) == Some('L') {
            delta *= -1;
        }

        pos += delta;
        while pos < 0 {
            pos += 100;
        }
        while pos > 99 {
            pos -= 100;
        }

        if pos == 0 {
            sum += 1;
        }
    }

    sum
}

pub fn day1_1(puzzle: String) -> i32 {
    let mut sum = 0;
    let mut pos = 50;

    for line in puzzle.lines() {
        let number = line.get(1..).unwrap();
        let delta = number.parse::<i32>().unwrap();

        let mut dir = 1;
        if line.chars().nth(0) == Some('L') {
            dir = -1;
        }

        sum += delta / 100;

        let mut over = false;
        let start_at_0 = pos == 0;

        pos += dir * (delta % 100);
        if pos < 0 {
            pos += 100;

            if !start_at_0 {
                sum +=1;
            }
        } else if pos > 99 {
            pos -= 100;
            sum += 1;
            over = true;
        }

        if !over && pos == 0 {
            sum += 1;
        }
    }

    sum
}
