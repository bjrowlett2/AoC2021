mod aoc;

#[derive(Clone)]
struct Day06 {
    timers: Vec<i64>,
}

fn main() {
    let mut day = Day06 {
        timers: vec![],
    };

    for line in aoc::lines("inputs/day_06.txt") {
        for timer in line.split(",") {
            match timer.parse::<i64>() {
                Ok(value) => day.timers.push(value),
                Err(reason) => panic!("String::parse failed: {}", reason),
            };
        }
    }

    match solve_part_1(&day) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("solve_part_1 failed: {}", reason),
    };

    match solve_part_2(&day) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    };
}

fn solve_part_1(day: &Day06) -> Result<i64, String> {
    let mut clone = day.clone();

    for _ in 0..80 {
        for i in 0..clone.timers.len() {
            clone.timers[i] -= 1;

            if clone.timers[i] < 0 {
                clone.timers[i] = 6;
                clone.timers.push(8);
            }
        }
    }

    return Ok(clone.timers.len() as i64);
}

fn solve_part_2(day: &Day06) -> Result<i64, String> {
    let mut histogram = [0; 9];
    for timer in &day.timers {
        histogram[*timer as usize] += 1;
    }

    for _ in 0..256 {
        histogram.rotate_left(1);
        histogram[6] += histogram[8];
    }

    let mut num_lanternfish = 0;
    for lanternfish in histogram {
        num_lanternfish += lanternfish;
    }

    return Ok(num_lanternfish);
}
