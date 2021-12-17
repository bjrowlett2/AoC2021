mod aoc;

use std::ops::RangeInclusive;

struct Day17 {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
}

fn parse_range(range: &str) -> RangeInclusive<i64> {
    let mut parts = range.split("..");
    let minimum = parts.next().unwrap().parse::<i64>().unwrap();
    let maximum = parts.next().unwrap().parse::<i64>().unwrap();

    return minimum..=maximum;
}

fn main() {
    let mut day = Day17 {
        x_range: 0..=0,
        y_range: 0..=0,
    };

    let target_area_prefix = "target area: ";
    for line in aoc::lines("inputs/day_17.txt") {
        if line.starts_with(target_area_prefix) {
            if let Some(target_area) = line.strip_prefix(target_area_prefix) {
                let mut parts = target_area.split(", ");

                if let Some(equation) = parts.next() {
                    if let Some(range) = equation.strip_prefix("x=") {
                        day.x_range = parse_range(range);
                    }
                }

                if let Some(equation) = parts.next() {
                    if let Some(range) = equation.strip_prefix("y=") {
                        day.y_range = parse_range(range);
                    }
                }
            }
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

type Point = (i64, i64);

struct State {
    position: Point,
    velocity: Point,
}

fn sign(value: i64) -> i64 {
    if value > 0 { return 1; }
    if value < 0 { return -1; }
    return 0;
}

fn step(state: &State) -> State {
    let x_position = state.position.0 + state.velocity.0;
    let y_position = state.position.1 + state.velocity.1;

    // Drag & Gravity
    let x_velocity = state.velocity.0 - sign(state.velocity.0);
    let y_velocity = state.velocity.1 - 1;

    return State {
        position: (x_position, y_position),
        velocity: (x_velocity, y_velocity),
    };
}

fn past_target(day: &Day17, state: &State) -> bool {
    return state.position.0 > *day.x_range.end()
        || state.position.1 < *day.y_range.start();
}

fn inside_target(day: &Day17, state: &State) -> bool {
    return day.x_range.contains(&state.position.0)
        && day.y_range.contains(&state.position.1);
}

fn solve_part_1(day: &Day17) -> Result<i64, String> {
    // Assume X is positive.
    let x_size = *day.x_range.end();

    // Assume Y is negative.
    let y_size = day.y_range.start().abs();

    let mut global_max_y = i64::MIN;
    for x_velocity in 0..=x_size {
        for y_velocity in -y_size..=y_size {
            let mut state = State {
                position: (0, 0),
                velocity: (x_velocity, y_velocity),
            };

            let mut max_y = i64::MIN;

            loop {
                state = step(&state);

                if past_target(&day, &state) {
                    break;
                }
        
                if state.position.1 > max_y {
                    max_y = state.position.1;
                }

                if inside_target(&day, &state) {
                    if max_y > global_max_y {
                        global_max_y = max_y;
                    }

                    break;
                }
            }
        }
    }

    return Ok(global_max_y);
}

fn solve_part_2(day: &Day17) -> Result<i64, String> {
    // Assume X is positive.
    let x_size = *day.x_range.end();

    // Assume Y is negative.
    let y_size = day.y_range.start().abs();

    let mut num_hits = 0;
    for x_velocity in 0..=x_size {
        for y_velocity in -y_size..=y_size {
            let mut state = State {
                position: (0, 0),
                velocity: (x_velocity, y_velocity),
            };

            loop {
                state = step(&state);

                if past_target(&day, &state) {
                    break;
                }

                if inside_target(&day, &state) {
                    num_hits += 1;
                    break;
                }
            }
        }
    }

    return Ok(num_hits);
}
