mod aoc;

use itertools::sorted;

struct Day07 {
    positions: Vec<i64>,
}

fn main() {
    let mut day = Day07 {
        positions: vec![],
    };

    for line in aoc::lines("inputs/day_07.txt") {
        for position in line.split(",") {
            match position.parse::<i64>() {
                Ok(value) => day.positions.push(value),
                Err(reason) => panic!("String::parse failed: {}", reason),
            };
        }
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

fn mean(vec: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for value in vec {
        sum += value;
    }

    return sum / vec.len() as i64;
}

fn median(vec: &Vec<i64>) -> i64 {
    let mut s = sorted(vec);
    return match s.nth(s.len() / 2) {
        Some(value) => *value,
        None => panic!("No median found"),
    };
}

fn solve_part_1(day: &Day07) -> Result<i64, String> {
    let target = median(&day.positions);

    let mut fuel = 0;
    for position in &day.positions {
        fuel += (position - target).abs();
    }

    return Ok(fuel);
}

fn solve_part_2(day: &Day07) -> Result<i64, String> {
    let target = mean(&day.positions);

    let mut fuel = 0;
    for position in &day.positions {
        let distance = (position - target).abs();
        fuel += distance * (distance + 1) / 2;
    }

    return Ok(fuel);
}
