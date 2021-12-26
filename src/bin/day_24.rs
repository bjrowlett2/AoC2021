mod aoc;

struct Day24 {
    dependencies: Vec<Dependency>,
}

enum Fsm {
    FindInputW,
    FindPushOrPop,
    ScanPushConstant,
    ExtractPushConstant,
    ExtractPopConstant,
}

struct State {
    inp: i64,
    constant: i64,
}

struct Dependency {
    source: usize,
    target: usize,
    constraint: i64,
}

fn main() {
    let mut day = Day24 {
        dependencies: vec![],
    };

    let mut states = vec![];
    
    let mut current_w = -1;
    let mut current_state = Fsm::FindInputW;
    for instruction in aoc::lines("inputs/day_24.txt") {
        match current_state {
            Fsm::FindInputW => {
                if instruction == "inp w" {
                    current_w += 1;
                    current_state = Fsm::FindPushOrPop;
                }
            },
            Fsm::FindPushOrPop => {
                if instruction == "div z 1" {
                    current_state = Fsm::ScanPushConstant;
                } else if instruction == "div z 26" {
                    current_state = Fsm::ExtractPopConstant;
                }
            },
            Fsm::ScanPushConstant => {
                // There are multiple `add y ...` instructions, but the
                // push constant is always immediately after the `add y w`.
                if instruction == "add y w" {
                    current_state = Fsm::ExtractPushConstant;
                }
            },
            Fsm::ExtractPushConstant => {
                if let Some(expr) = instruction.strip_prefix("add y ") {
                    current_state = Fsm::FindInputW;

                    let constant = match expr.parse::<i64>() {
                        Ok(value) => value,
                        Err(_) => panic!("No push constant found"),
                    };

                    states.push(State {
                        inp: current_w,
                        constant: constant,
                    });
                }
            },
            Fsm::ExtractPopConstant => {
                if let Some(expr) = instruction.strip_prefix("add x ") {
                    let constant = match expr.parse::<i64>() {
                        Ok(value) => value,
                        Err(_) => panic!("No pop constant found"),
                    };
    
                    if let Some(top) = states.pop() {
                        current_state = Fsm::FindInputW;

                        day.dependencies.push(Dependency {
                            source: top.inp as usize,
                            target: current_w as usize,
                            constraint: top.constant + constant,
                        });
                    }
                }
            },
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

fn solve_part_1(day: &Day24) -> Result<i64, String> {
    let mut digits = vec![0; 14];
    for dependency in &day.dependencies {
        for i in (1..=9).rev() { // Iterate from highest to lowest.
            let w = i + dependency.constraint;
            if (1 <= w) && (w <= 9) {
                digits[dependency.source] = i;
                digits[dependency.target] = w;
                break;
            }
        }
    }

    let mut model_number =  0;
    for digit in &digits {
        model_number *= 10;
        model_number += digit;
    }

    return Ok(model_number);
}

fn solve_part_2(day: &Day24) -> Result<i64, String> {
    let mut digits = vec![0; 14];
    for dependency in &day.dependencies {
        for i in 1..=9 { // Iterate from lowest to highest.
            let w = i + dependency.constraint;
            if (1 <= w) && (w <= 9) {
                digits[dependency.source] = i;
                digits[dependency.target] = w;
                break;
            }
        }
    }

    let mut model_number =  0;
    for digit in &digits {
        model_number *= 10;
        model_number += digit;
    }

    return Ok(model_number);
}
