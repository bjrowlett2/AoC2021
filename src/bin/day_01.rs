mod aoc;

struct Day01 {
    depths: Vec<i32>,
}

fn main() {
    let mut day = Day01 {
        depths: vec![],
    };

    for line in aoc::lines("inputs/day_01.txt") {
        match line.parse::<i32>() {
            Ok(value) => day.depths.push(value),
            Err(reason) => panic!("String::parse failed: {}", reason),
        };
    }

    match solve_part_1(&day) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("Part 1 failed: {}", reason),
    };

    match solve_part_2(&day) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("Part 2 failed: {}", reason),
    };
}

fn count_measurement_increases(values: &Vec<i32>) -> i32 {
    let mut increases = 0;
    for i in 1..values.len() {
        if values[i] > values[i - 1] {
            increases += 1;
        }
    }

    return increases;
}

fn solve_part_1(day: &Day01) -> Result<i32, String> {
    return Ok(count_measurement_increases(&day.depths));
}

fn solve_part_2(day: &Day01) -> Result<i32, String> {
    let mut windows = vec![];
    for i in 0..(day.depths.len() - 2) {
        windows.push(day.depths[i] + day.depths[i + 1] + day.depths[i + 2]);
    }

    return Ok(count_measurement_increases(&windows));
}
