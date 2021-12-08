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
    };

    match solve_part_2(&day) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    };
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

fn except(this: &String, that: &String) -> String {
    let mut result = String::new();
    for ch in this.chars() {
        if !that.contains(ch) {
            result.push(ch);
        }
    }

    return result;
}

fn overlap(this: &String, that: &String) -> i64 {
    let mut overlap = 0;
    for ch1 in this.chars() {
        for ch2 in that.chars() {
            if ch1 == ch2 {
                overlap += 1;
            }
        }
    }

    return overlap;
}

// The following code is terrible and I feel bad.
fn solve_part_2(day: &Day08) -> Result<i64, String> {
    let mut sum = 0;
    for signal in &day.signals {
        let mut patterns = signal.patterns.clone();
        patterns.sort_by_key(|pattern| pattern.len());

        let mut segment_possibilities = vec![];
        for _ in 0..7 {
            segment_possibilities.push(String::new());
        }

        let mut digits = HashMap::<&String, i64>::new();

        // 1
        digits.insert(&patterns[0], 1);
        
        segment_possibilities[2] = patterns[0].to_string();
        segment_possibilities[5] = patterns[0].to_string();

        // 7
        digits.insert(&patterns[1], 7);

        segment_possibilities[0] = except(&patterns[1], &patterns[0]);

        // 4
        digits.insert(&patterns[2], 4);

        segment_possibilities[1] = except(&patterns[2], &segment_possibilities[2]);
        segment_possibilities[1] = except(&segment_possibilities[1], &segment_possibilities[5]);
        segment_possibilities[3] = except(&patterns[2], &segment_possibilities[2]);
        segment_possibilities[3] = except(&segment_possibilities[3], &segment_possibilities[5]);

        // 8
        digits.insert(&patterns[9], 8);

        // 0
        for pattern in &patterns {
            if pattern.len() == 6 {
                if overlap(&pattern, &segment_possibilities[3]) == 1 {
                    digits.insert(&pattern, 0);

                    segment_possibilities[3] = except(&segment_possibilities[3], &pattern);
                    segment_possibilities[1] = except(&segment_possibilities[1], &segment_possibilities[3]);

                    break;
                }
            }
        }

        // 5
        for pattern in &patterns {
            if pattern.len() == 5 {
                if pattern.contains(&segment_possibilities[1]) {
                    digits.insert(&pattern, 5);

                    segment_possibilities[2] = except(&segment_possibilities[2], &pattern);
                    segment_possibilities[5] = except(&segment_possibilities[5], &segment_possibilities[2]);

                    break;
                }
            }
        }

        // 9
        for pattern in &patterns {
            if pattern.len() == 6 {
                if pattern.contains(&segment_possibilities[2]) {
                    if pattern.contains(&segment_possibilities[3]) {
                        digits.insert(&pattern, 9);

                        segment_possibilities[4] = except(&String::from("abcdefg"), &pattern);

                        segment_possibilities[6] = except(&String::from("abcdefg"), &segment_possibilities[0]);
                        segment_possibilities[6] = except(&segment_possibilities[6], &segment_possibilities[1]);
                        segment_possibilities[6] = except(&segment_possibilities[6], &segment_possibilities[2]);
                        segment_possibilities[6] = except(&segment_possibilities[6], &segment_possibilities[3]);
                        segment_possibilities[6] = except(&segment_possibilities[6], &segment_possibilities[4]);
                        segment_possibilities[6] = except(&segment_possibilities[6], &segment_possibilities[5]);

                        break;
                    }
                }
            }
        }
        
        for pattern in &patterns {
            // 2, 3
            if pattern.len() == 5 {
                if pattern.contains(&segment_possibilities[2]) {
                    if pattern.contains(&segment_possibilities[4]) {
                        digits.insert(&pattern, 2);
                    }
                }

                if pattern.contains(&segment_possibilities[2]) {
                    if pattern.contains(&segment_possibilities[5]) {
                        digits.insert(&pattern, 3);
                    }
                }
            }
            
            // 6
            if pattern.len() == 6 {
                if pattern.contains(&segment_possibilities[3]) {
                    if pattern.contains(&segment_possibilities[4]) {
                        digits.insert(&pattern, 6);
                    }
                }
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
