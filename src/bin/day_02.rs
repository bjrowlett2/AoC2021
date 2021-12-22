mod aoc;

enum Action {
    Up,
    Down,
    Forward,
}

struct Command {
    action: Action,
    amount: i64,
}

struct Day02 {
    commands: Vec<Command>,
}

fn main() {
    let mut day = Day02 {
        commands: vec![],
    };

    for line in aoc::lines("inputs/day_02.txt") {
        day.commands.push(parse_command(&line));
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

fn parse_action(action: &str) -> Action {
    return match action {
        "up" => Action::Up,
        "down" => Action::Down,
        "forward" => Action::Forward,
        unknown => panic!("parse_action failed: {}", unknown),
    };
}

fn parse_amount(amount: &str) -> i64 {
    return match amount.parse::<i64>() {
        Ok(value) => value,
        Err(reason) => panic!("String::parse failed: {}", reason),
    };
}

fn parse_command(line: &str) -> Command {
    let mut parts = line.split(" ");

    return Command {
        action: match parts.next() {
            Some(value) => parse_action(&value),
            None => panic!("parse_command failed: No action found"),
        },
        amount: match parts.next() {
            Some(value) => parse_amount(&value),
            None => panic!("parse_command failed: No amount found"),
        },
    };
}

fn solve_part_1(day: &Day02) -> Result<i64, String> {
    let mut depth = 0;
    let mut horizontal = 0;
    
    for command in &day.commands {
        match command.action {
            Action::Up => depth -= command.amount,
            Action::Down => depth += command.amount,
            Action::Forward => horizontal += command.amount,
        };
    }

    return Ok(depth * horizontal);
}

fn solve_part_2(day: &Day02) -> Result<i64, String> {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    for command in &day.commands {
        match command.action {
            Action::Up => {
                aim -= command.amount;
            },
            Action::Down => {
                aim += command.amount;
            },
            Action::Forward => {
                horizontal += command.amount;
                depth += aim * command.amount;
            },
        };
    }

    return Ok(depth * horizontal);
}
