mod aoc;

struct Day01 {
    depths: Vec<i64>,
}

fn main() {
    let mut day = Day01 {
        depths: vec![],
    };

    for line in aoc::lines("inputs/day_01.txt") {
        match line.parse::<i64>() {
            Ok(value) => day.depths.push(value),
            Err(reason) => panic!("String::parse failed: {}", reason),
        };
    }

    match solve_part_1(&day) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("solve_part_1 failed: {}", reason),
    }

    match solve_part_2(&day) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    }
}

fn count_measurement_increases(values: &Vec<i64>) -> i64 {
    let mut increases = 0;
    for i in 1..values.len() {
        if values[i] > values[i - 1] {
            increases += 1;
        }
    }

    return increases;
}

fn solve_part_1(day: &Day01) -> Result<i64, String> {
    return Ok(count_measurement_increases(&day.depths));
}

fn solve_part_2(day: &Day01) -> Result<i64, String> {
    let mut windows = vec![];
    for values in day.depths.windows(3) {
        windows.push(values[0] + values[1] + values[2]);
    }

    return Ok(count_measurement_increases(&windows));
}
