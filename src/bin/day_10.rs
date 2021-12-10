mod aoc;

use std::collections::HashMap;

struct Day10 {
    lines: Vec<String>,
}

fn main() {
    let mut day = Day10 {
        lines: vec![],
    };

    for line in aoc::lines("inputs/day_10.txt") {
        day.lines.push(line);
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

enum Status {
    Complete,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn is_open(symbol: char) -> bool {
    return symbol == '('
        || symbol == '['
        || symbol == '{'
        || symbol == '<';
}

fn is_close_for(open: char, close: char) -> bool {
    return (open == '(' && close == ')')
        || (open == '[' && close == ']')
        || (open == '{' && close == '}')
        || (open == '<' && close == '>');
}

fn determine_status(line: &String) -> Status {
    let mut stack = vec![];
    for symbol in line.chars() {
        if is_open(symbol) {
            stack.push(symbol);
        } else {
            let open = match stack.pop() {
                Some(value) => value,
                None => panic!("Tried to pop, but no value found"),
            };

            if !is_close_for(open, symbol) {
                return Status::Corrupted(symbol);
            }
        }
    }

    if !stack.is_empty() {
        return Status::Incomplete(stack);
    }
    
    return Status::Complete;
}

fn error_score_for(symbol: char) -> i64 {
    let scores = HashMap::from([
        (')', 3), (']', 57), ('}', 1197), ('>', 25137),
    ]);

    return match scores.get(&symbol) {
        Some(value) => *value,
        None => panic!("Unknown symbol found: `{}`", symbol),
    };
}

fn solve_part_1(day: &Day10) -> Result<i64, String> {
    let mut error_score = 0;
    for line in &day.lines {
        match determine_status(line) {
            Status::Complete => {},
            Status::Incomplete(_) => {},
            Status::Corrupted(symbol) => {
                error_score += error_score_for(symbol);
            },
        };
    }

    return Ok(error_score);
}

fn bump(score: &mut i64, symbol: char) {
    // Skip finding the matching close character,
    // score based on the open characters directly.
    let scores = HashMap::from([
        ('(', 1), ('[', 2), ('{', 3), ('<', 4),
    ]);

    *score *= 5;
    *score += match scores.get(&symbol) {
        Some(value) => *value,
        None => panic!("Unknown symbol found: `{}`", symbol),
    };
}

fn autocomplete_score_for(stack: &mut Vec<char>) -> i64 {
    let mut score = 0;
    while !stack.is_empty() {
        match stack.pop() {
            Some(symbol) => bump(&mut score, symbol),
            None => panic!("Tried to pop, but no value found"),
        };
    }

    return score;
}

fn solve_part_2(day: &Day10) -> Result<i64, String> {
    let mut scores = vec![];
    for line in &day.lines {
        match determine_status(line) {
            Status::Complete => {},
            Status::Corrupted(_) => {},
            Status::Incomplete(ref mut stack) => {
                scores.push(autocomplete_score_for(stack));
            },
        };
    }

    scores.sort();

    let n = scores.len();
    return Ok(scores[n / 2]);
}
