mod aoc;

#[derive(Clone)]
struct State {
    value: i64,
    level: i64,
}

type SnailfishNumber = Vec<State>;

#[derive(Clone)]
struct Day18 {
    snailfish_numbers: Vec<SnailfishNumber>,
}

fn main() {
    let mut day = Day18 {
        snailfish_numbers: vec![],
    };
    
    for line in aoc::lines("inputs/day_18.txt") {
        day.snailfish_numbers.push(parse_snailfish_number(&line));
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

fn parse_snailfish_number(line: &String) -> SnailfishNumber {
    let mut current_level = 0;
    let mut snailfish_number = vec![];

    for ch in line.chars() {
        if ch == '[' {
            current_level += 1;
        } else if ch == ']' {
            current_level -= 1;
        } else if let Some(digit) = ch.to_digit(10) {
            snailfish_number.push(State {
                value: digit as i64,
                level: current_level,
            });
        }
    }

    return snailfish_number;
}

fn reduce(snailfish_number: &mut SnailfishNumber) {
    loop {
        while explode(snailfish_number) {
            continue;
        }

        if split(snailfish_number) {
            continue;
        }

        break;
    }
}

fn add(acc: &mut SnailfishNumber, rhs: &SnailfishNumber) {
    for i in 0..acc.len() {
        acc[i].level += 1;
    }

    for i in 0..rhs.len() {
        acc.push(State {
            value: rhs[i].value,
            level: rhs[i].level + 1,
        });
    }
}

fn explode(snailfish_number: &mut SnailfishNumber) -> bool {
    for i in 0..(snailfish_number.len() - 1) {
        let lhs = snailfish_number[i].clone();
        let rhs = snailfish_number[i + 1].clone();

        if (lhs.level > 4) && (rhs.level > 4) {
            if i > 0 {
                snailfish_number[i - 1].value += lhs.value;
            }

            if i < (snailfish_number.len() - 2) {
                snailfish_number[i + 2].value += rhs.value;
            }

            snailfish_number[i].value = 0;
            snailfish_number[i].level -= 1;
            snailfish_number.remove(i + 1);
            return true;
        }
    }

    return false;
}

fn split(snailfish_number: &mut SnailfishNumber) -> bool {
    for i in 0..snailfish_number.len() {
        if snailfish_number[i].value >= 10 {
            let value = snailfish_number[i].value;

            snailfish_number[i].value = value / 2;
            snailfish_number[i].level += 1;

            let new_state = State {
                value: (value / 2) + (value % 2),
                level: snailfish_number[i].level,
            };

            snailfish_number.insert(i + 1, new_state);
            return true;
        }
    }

    return false;
}

fn magnitude(i: &mut usize, level: i64, snailfish_number: &SnailfishNumber) -> i64 {
    let mut result = 0;
    if snailfish_number[*i].level == level {
        result += 3 * snailfish_number[*i].value;
        *i += 1;
    } else {
        result += 3 * magnitude(i, level + 1, snailfish_number);
    }

    if snailfish_number[*i].level == level {
        result += 2 * snailfish_number[*i].value;
        *i += 1;
    } else {
        result += 2 * magnitude(i, level + 1, snailfish_number);
    }

    return result;
}

fn solve_part_1(day: &Day18) -> Result<i64, String> {
    let mut result = day.snailfish_numbers[0].clone();

    for i in 1..day.snailfish_numbers.len() {
        add(&mut result, &day.snailfish_numbers[i]);
        reduce(&mut result);
    }

    return Ok(magnitude(&mut 0, 1, &result));
}

fn solve_part_2(day: &Day18) -> Result<i64, String> {
    let mut max_magnitude = i64::MIN;
    for i in 0..day.snailfish_numbers.len() {
        for j in i..day.snailfish_numbers.len() {
            let mut result_1 = day.snailfish_numbers[i].clone();

            add(&mut result_1, &day.snailfish_numbers[j]);
            reduce(&mut result_1);

            let magnitude_1 = magnitude(&mut 0, 1, &result_1);
            if magnitude_1 > max_magnitude {
                max_magnitude = magnitude_1;
            }

            // Not commutative, swap order.
            let mut result_2 = day.snailfish_numbers[j].clone();
            add(&mut result_2, &day.snailfish_numbers[i]);
            reduce(&mut result_2);

            let magnitude_2 = magnitude(&mut 0, 1, &result_2);
            if magnitude_2 > max_magnitude {
                max_magnitude = magnitude_2;
            }
        }
    }
    
    return Ok(max_magnitude);
}
