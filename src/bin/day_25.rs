mod aoc;

type SeaFloor = Vec<Vec<char>>;

struct Day25 {
    sea_floor: SeaFloor,
}

fn main() {
    let mut day = Day25 {
        sea_floor: vec![],
    };

    for line in aoc::lines("inputs/day_25.txt") {
        day.sea_floor.push(vec![]);
        for ch in line.chars() {
            match day.sea_floor.last_mut() {
                Some(last) => last.push(ch),
                None => panic!("Last row not found"),
            }
        }
    }

    match solve_part_1(&mut day) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("solve_part_1 failed: {}", reason),
    }
}

struct Movement {
    source: (usize, usize),
    target: (usize, usize),
}

impl Movement {
    fn from(x0: usize, y0: usize, x1: usize, y1: usize) -> Movement {
        return Movement {
            source: (x0, y0),
            target: (x1, y1),
        };
    }
}

fn step(sea_floor: &mut SeaFloor) -> bool {
    let mut moved = false;
    moved |= step_east(sea_floor);
    moved |= step_south(sea_floor);
    return moved;
}

fn step_east(sea_floor: &mut SeaFloor) -> bool {
    let mut movements = vec![];
    for y in 0..sea_floor.len() {
        let y0 = y % sea_floor.len();

        for x in 0..sea_floor[y].len() {
            let x0 = x % sea_floor[y0].len();
            let x1 = (x + 1) % sea_floor[y0].len();

            if (sea_floor[y0][x0] == '>') && (sea_floor[y0][x1] == '.') {
                movements.push(Movement::from(x0, y0, x1, y0));
            }
        }
    }

    for movement in &movements {
        let sea_cucumber = sea_floor[movement.source.1][movement.source.0];
        sea_floor[movement.target.1][movement.target.0] = sea_cucumber;
        sea_floor[movement.source.1][movement.source.0] = '.';
    }

    return !movements.is_empty();
}

fn step_south(sea_floor: &mut SeaFloor) -> bool {
    let mut movements = vec![];
    for y in 0..sea_floor.len() {
        let y0 = y % sea_floor.len();
        let y1 = (y + 1) % sea_floor.len();

        for x in 0..sea_floor[y].len() {
            let x0 = x % sea_floor[y0].len();

            if (sea_floor[y0][x0] == 'v') && (sea_floor[y1][x0] == '.') {
                movements.push(Movement::from(x0, y0, x0, y1));
            }
        }
    }

    for movement in &movements {
        let sea_cucumber = sea_floor[movement.source.1][movement.source.0];
        sea_floor[movement.target.1][movement.target.0] = sea_cucumber;
        sea_floor[movement.source.1][movement.source.0] = '.';
    }

    return !movements.is_empty();
}

fn solve_part_1(day: &mut Day25) -> Result<i64, String> {
    let mut steps = 1;
    while step(&mut day.sea_floor) {
        steps += 1;
    }

    return Ok(steps);
}
