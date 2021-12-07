mod aoc;

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
    };

    match solve_part_2(&day) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    };
}

fn solve_part_1(day: &Day07) -> Result<i64, String> {
    let mut minimum = i64::MAX;
    let mut maximum = i64::MIN;
    for position in &day.positions {
        if *position < minimum {
            minimum = *position;
        }

        if *position > maximum {
            maximum = *position;
        }
    }

    let mut total_fuel = i64::MAX;
    for target in minimum..(maximum + 1) {
        let mut fuel = 0;
        for position in &day.positions {
            fuel += (position - target).abs();
        }

        if fuel < total_fuel {
            total_fuel = fuel;
        }
    }

    return Ok(total_fuel);
}

fn solve_part_2(day: &Day07) -> Result<i64, String> {
    let mut minimum = i64::MAX;
    let mut maximum = i64::MIN;
    for position in &day.positions {
        if *position < minimum {
            minimum = *position;
        }

        if *position > maximum {
            maximum = *position;
        }
    }

    let mut total_fuel = i64::MAX;
    for target in minimum..(maximum + 1) {
        let mut fuel = 0;
        for position in &day.positions {
            let distance = (position - target).abs();
            fuel += distance * (distance + 1) / 2;
        }

        if fuel < total_fuel {
            total_fuel = fuel;
        }
    }

    return Ok(total_fuel);
}
