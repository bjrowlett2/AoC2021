mod aoc;

use std::collections::HashSet;

#[derive(Clone)]
enum Axis {
    X, Y
}

#[derive(Clone)]
struct FoldAlong {
    axis: Axis,
    value: i64,
}

type Point = (i64, i64);

#[derive(Clone)]
struct Day13 {
    points: HashSet<Point>,
    fold_alongs: Vec<FoldAlong>,
}

fn main() {
    let mut day = Day13 {
        points: HashSet::new(),
        fold_alongs: vec![],
    };

    for line in aoc::lines("inputs/day_13.txt") {
        if let Some(point) = parse_point(&line) {
            day.points.insert(point);
        }

        if let Some(fold_along) = parse_fold_along(&line) {
            day.fold_alongs.push(fold_along);
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

fn parse_value(input: &str) -> i64 {
    return match input.parse::<i64>() {
        Ok(value) => value,
        Err(reason) => panic!("String::parse failed: {}", reason),
    };
}

fn parse_point(line: &String) -> Option<Point> {
    if line.contains(",") {
        let mut parts = line.split(",");

        let x = match parts.next() {
            Some(value) => parse_value(value),
            None => panic!("No x coordinate found"),
        };

        let y = match parts.next() {
            Some(value) => parse_value(value),
            None => panic!("No y coordinate found"),
        };

        return Some((x, y));
    }

    return None;
}

fn parse_axis(input: &str) -> Axis {
    return match input {
        "x" => Axis::X,
        "y" => Axis::Y,
        unknown => panic!("Unknown axis: {}", unknown),
    };
}

fn parse_fold_along(line: &String) -> Option<FoldAlong> {
    if line.starts_with("fold along ") {
        if let Some(equ) = line.strip_prefix("fold along ") {
            let mut parts = equ.split("=");

            let fold_along = FoldAlong {
                axis: match parts.next() {
                    Some(value) => parse_axis(value),
                    None => panic!("No fold along axis found"),
                },
                value: match parts.next() {
                    Some(value) => parse_value(value),
                    None => panic!("No fold along value found"),
                },
            };

            return Some(fold_along);
        }
    }

    return None;
}

fn fold_x(points: &mut HashSet<Point>, value: i64) {
    let mut moved_by_fold = vec![];
    for point in points.iter() {
        if point.0 > value {
            moved_by_fold.push(*point);
        }
    }

    points.retain(|p| p.0 < value);
    
    for point in &moved_by_fold {
        let delta = point.0 - value;
        points.insert((value - delta, point.1));
    }
}

fn fold_y(points: &mut HashSet<Point>, value: i64) {
    let mut moved_by_fold = vec![];
    for point in points.iter() {
        if point.1 > value {
            moved_by_fold.push(*point);
        }
    }

    points.retain(|p| p.1 < value);
    
    for point in &moved_by_fold {
        let delta = point.1 - value;
        points.insert((point.0, value - delta));
    }
}

fn solve_part_1(day: &Day13) -> Result<i64, String> {
    let mut clone = day.clone();
    if !clone.fold_alongs.is_empty() {
        let fold_along = &clone.fold_alongs[0];

        match fold_along.axis {
            Axis::X => fold_x(&mut clone.points, fold_along.value),
            Axis::Y => fold_y(&mut clone.points, fold_along.value),
        };
    }

    return Ok(clone.points.len() as i64);
}

fn calc_width(points: &HashSet<Point>) -> usize {
    return match points.iter().max_by_key(|p| p.0) {
        Some(value) => (value.0 + 1) as usize,
        None => panic!("No maximum x value found"),
    };
}

fn calc_height(points: &HashSet<Point>) -> usize {
    return match points.iter().max_by_key(|p| p.1) {
        Some(value) => (value.1 + 1) as usize,
        None => panic!("No maximum y value found"),
    };
}

fn capture(points: &HashSet<Point>) -> String {
    let width = calc_width(points);
    let height = calc_height(points);
    let mut sensor = vec![vec![' '; width]; height];

    for point in points {
        sensor[point.1 as usize][point.0 as usize] = '█';
    }

    let mut output = String::new();
    for y in 0..height {
        output.push('\n');
        for x in 0..width {
            output.push(sensor[y][x]);
        }
    }

    return output;
}

fn solve_part_2(day: &Day13) -> Result<String, String> {
    let mut clone = day.clone();
    for fold_along in &clone.fold_alongs {
        match fold_along.axis {
            Axis::X => fold_x(&mut clone.points, fold_along.value),
            Axis::Y => fold_y(&mut clone.points, fold_along.value),
        };
    }

    return Ok(capture(&clone.points));
}
