mod aoc;

use itertools::sorted;
use std::collections::HashMap;

struct Signal {
    patterns: Vec<String>,
    output_values: Vec<String>,
}

struct Day08 {
    signals: Vec<Signal>,
}

fn main() {
    let mut day = Day08 {
        signals: vec![],
    };

    for line in aoc::lines("inputs/day_08.txt") {
        day.signals.push(parse_signal(&line));
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

fn parse_segments(segments: &str) -> Vec<String> {
    let mut parts = segments.split(" ");

    let mut values = vec![];
    while let Some(value) = parts.next() {
        let chars = sorted(value.chars());
        values.push(chars.collect());
    }

    return values;
}

fn parse_signal(line: &String) -> Signal {
    let mut parts = line.split(" | ");

    return Signal {
        patterns: match parts.next() {
            Some(value) => parse_segments(value),
            None => panic!("No signal patterns found"),
        },
        output_values: match parts.next() {
            Some(value) => parse_segments(value),
            None => panic!("No output values found"),
        },
    };
}

fn solve_part_1(day: &Day08) -> Result<i64, String> {
    let output_value_lengths = vec![2, 4, 3, 7];

    let mut count = 0;
    for signal in &day.signals {
        for output_value in &signal.output_values {
            let length = output_value.len() as i64;
            if output_value_lengths.contains(&length) {
                count += 1;
            }
        }
    }

    return Ok(count);
}

fn except(minuend: &String, subtrahend: &String) -> String {
    let mut difference = String::new();
    for segment in minuend.chars() {
        if !subtrahend.contains(segment) {
            difference.push(segment);
        }
    }

    return difference;
}

fn solve_part_2(day: &Day08) -> Result<i64, String> {
    let mut sum = 0;
    for signal in &day.signals {
        let mut patterns = signal.patterns.clone();
        patterns.sort_by_key(|pattern| pattern.len());

        let mut digits = HashMap::<&String, i64>::new();

        digits.insert(&patterns[0], 1);
        digits.insert(&patterns[2], 4);
        digits.insert(&patterns[1], 7);
        digits.insert(&patterns[9], 8);

        for pattern in &patterns {
            let minus_1 = except(&pattern, &patterns[0]);
            let minus_4 = except(&pattern, &patterns[2]);

            if pattern.len() == 5 {
                match (minus_1.len(), minus_4.len()) {
                    (4, 3) => digits.insert(&pattern, 2),
                    (3, 2) => digits.insert(&pattern, 3),
                    (4, 2) => digits.insert(&pattern, 5),
                    _ => panic!("No matching length tuple found"),
                };
            } else if pattern.len() == 6 {
                match (minus_1.len(), minus_4.len()) {
                    (4, 3) => digits.insert(&pattern, 0),
                    (5, 3) => digits.insert(&pattern, 6),
                    (4, 2) => digits.insert(&pattern, 9),
                    _ => panic!("No matching length tuple found"),
                };
            }
        }

        let mut partial_sum = 0;
        for output_value in &signal.output_values {
            partial_sum *= 10;
            partial_sum += match digits.get(output_value) {
                Some(value) => value,
                None => panic!("No digit found"),
            };
        }

        sum += partial_sum;
    }

    return Ok(sum);
}
