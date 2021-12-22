mod aoc;

use std::cmp::min;
use std::cmp::max;

struct Day22 {
    reboot_steps: Vec<RebootStep>,
}

#[derive(Clone)]
struct Region {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

struct RebootStep {
    action: String,
    region: Region,
}

fn parse_int(value: &str) -> i64 {
    return match value.parse::<i64>() {
        Ok(value) => value,
        Err(reason) => panic!("String::parse failed: {}", reason),
    };
}

fn parse_range(range: &str) -> (i64, i64) {
    let mut parts = range.split("..");

    let lower_bound = match parts.next() {
        Some(value) => parse_int(value),
        None => panic!("No lower bound found"),
    };

    let upper_bound = match parts.next() {
        Some(value) => parse_int(value),
        None => panic!("No upper bound found"),
    };

    return (lower_bound, upper_bound);
}

fn main() {
    let mut day = Day22 {
        reboot_steps: vec![],
    };

    for line in aoc::lines("inputs/day_22.txt") {
        let mut parts = line.split(" ");

        let mut reboot_step = RebootStep {
            action: match parts.next() {
                Some(value) => value.to_string(),
                None => panic!("No action found"),
            },
            region: Region {
                x: (0, 0), y: (0, 0), z: (0, 0),
            },
        };

        if let Some(ranges) = parts.next() {
            let mut parts = ranges.split(",");

            if let Some(equation) = parts.next() {
                if let Some(value) = equation.strip_prefix("x=") {
                    reboot_step.region.x = parse_range(value);
                }
            }

            if let Some(equation) = parts.next() {
                if let Some(value) = equation.strip_prefix("y=") {
                    reboot_step.region.y = parse_range(value);
                }
            }

            if let Some(equation) = parts.next() {
                if let Some(value) = equation.strip_prefix("z=") {
                    reboot_step.region.z = parse_range(value);
                }
            }

            day.reboot_steps.push(reboot_step);
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

struct State {
    count: i64,
    region: Region,
}

impl State {
    fn from(count: i64, region: &Region) -> State {
        return State {
            count: count,
            region: region.clone(),
        };
    }
}

fn has_overlap(first: (i64, i64), second: (i64, i64)) -> bool {
    return (first.0 <= second.1) && (first.1 >= second.0);
}

fn find_overlap(first: &Region, second: &Region) -> Option<Region> {
    if has_overlap(first.x, second.x) {
        if has_overlap(first.y, second.y) {
            if has_overlap(first.z, second.z) {
                let common = Region {
                    x: (max(first.x.0, second.x.0), min(first.x.1, second.x.1)),
                    y: (max(first.y.0, second.y.0), min(first.y.1, second.y.1)),
                    z: (max(first.z.0, second.z.0), min(first.z.1, second.z.1)),
                };
            
                return Some(common);
            }
        }
    }

    return None;
}

fn volume(region: &Region) -> i64 {
    return (region.x.1 - region.x.0 + 1).abs()
         * (region.y.1 - region.y.0 + 1).abs()
         * (region.z.1 - region.z.0 + 1).abs();
}

fn apply_step(reboot_step: &RebootStep, states: &mut Vec<State>) {
    let mut overlaps = vec![];
    for state in states.iter() {
        if let Some(common) = find_overlap(&state.region, &reboot_step.region) {
            // This accounts for both "on" double covers and "off" subtractions.
            overlaps.push(State::from(-1 * state.count, &common));
        }
    }

    for state in overlaps {
        states.push(state);
    }

    if reboot_step.action == "on" {
        states.push(State::from(1, &reboot_step.region));
    }
}

fn solve_part_1(day: &Day22) -> Result<i64, String> {
    let mut states: Vec<State> = vec![];
    for reboot_step in &day.reboot_steps {
        apply_step(&reboot_step, &mut states);
    }

    let consider_region = Region {
        x: (-50, 50), y: (-50, 50), z: (-50, 50),
    };

    let mut count = 0;
    for state in &states {
        if let Some(common) = find_overlap(&state.region, &consider_region) {
            count += state.count * volume(&common);
        }
    }

    return Ok(count);
}

fn solve_part_2(day: &Day22) -> Result<i64, String> {
    let mut states: Vec<State> = vec![];
    for reboot_step in &day.reboot_steps {
        apply_step(&reboot_step, &mut states);
    }

    let mut count = 0;
    for state in &states {
        count += state.count * volume(&state.region);
    }

    return Ok(count);
}
