mod aoc;

use std::collections::VecDeque;

type Point = (usize, usize);

struct Day15 {
    risk_levels: Vec<Vec<i64>>,
}

fn main() {
    let mut day = Day15 {
        risk_levels: vec![],
    };

    for line in aoc::lines("inputs/day_15.txt") {
        day.risk_levels.push(vec![]);

        let last = day.risk_levels.len() - 1;
        for ch in line.chars() {
            let s = ch.to_string();
            let risk_level = match s.parse::<i64>() {
                Ok(value) => value,
                Err(reason) => panic!("String::parse failed: {}", reason),
            };

            day.risk_levels[last].push(risk_level);
        }
    }

    for i in 0..day.risk_levels.len() {
        assert_eq!(day.risk_levels.len(), day.risk_levels[i].len());
    }

    match solve_part_1(&day) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("solve_part_1 failed: {}", reason),
    }

    match solve_part_2(&mut day) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    }
}

fn neighbors(day: &Day15, u: &Point) -> Vec<Point> {
    let mut neighbors = vec![
        (u.0 - 1, u.1), (u.0 + 1, u.1),
        (u.0, u.1 - 1), (u.0, u.1 + 1),
    ];

    let size = day.risk_levels.len();

    neighbors.retain(|point| {
        return (0..size).contains(&point.0)
            && (0..size).contains(&point.1);
    });

    return neighbors;
}

fn navigate(day: &Day15) -> i64 {
    let mut queue = VecDeque::new();

    let size = day.risk_levels.len();
    let mut distances = vec![vec![i64::MAX; size]; size];

    distances[0][0] = 0;
    queue.push_back((0, 0));
    while !queue.is_empty() {
        if let Some(node) = queue.pop_front() {
            let distance = distances[node.1][node.0];

            for neighbor in neighbors(&day, &node) {
                let risk_level = day.risk_levels[neighbor.1][neighbor.0];

                let cost = distance + risk_level;
                if cost < distances[neighbor.1][neighbor.0] {
                    queue.push_back(neighbor);
                    distances[neighbor.1][neighbor.0] = cost;
                }
            }
        }
    }

    return distances[size - 1][size - 1];
}

fn solve_part_1(day: &Day15) -> Result<i64, String> {
    return Ok(navigate(&day));
}

fn wrap(value: i64) -> i64 {
    if value > 9 {
        return value - 9;
    }

    return value;
}

fn solve_part_2(day: &mut Day15) -> Result<i64, String> {
    let original_size = day.risk_levels.len();

    for tile in 1..5 {// Repeat the map horizontally 5 times.
        for y in 0..original_size {
            for x in 0..original_size {
                let risk_level = day.risk_levels[y][x];
                let higher_risk_level = risk_level + tile;
                day.risk_levels[y].push(wrap(higher_risk_level));
            }
        }
    }

    for tile in 1..5 { // Repeat the map vertically 5 times.
        for y in 0..original_size {
            day.risk_levels.push(vec![]);

            let last = day.risk_levels.len() - 1;
            for x in 0..day.risk_levels[y].len() {
                let risk_level = day.risk_levels[y][x];
                let higher_risk_level = risk_level + tile;
                day.risk_levels[last].push(wrap(higher_risk_level));
            }
        }
    }

    return Ok(navigate(&day));
}
