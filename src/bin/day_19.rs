mod aoc;

use std::collections::HashSet;
use std::collections::VecDeque;

struct Day19 {
    scanners: Vec<Scanner>,
}

type Point = (i64, i64, i64);

struct Scanner {
    beacons: Vec<Point>,
}

impl Scanner {
    fn new() -> Scanner {
        return Scanner {
            beacons: vec![],
        };
    }

    fn add_beacon(&mut self, line: &String) {
        let mut parts = line.split(",");

        let mut x = 0;
        if let Some(coord) = parts.next() {
            x = coord.parse::<i64>().unwrap();
        }
        
        let mut y = 0;
        if let Some(coord) = parts.next() {
            y = coord.parse::<i64>().unwrap();
        }
        
        let mut z = 0;
        if let Some(coord) = parts.next() {
            z = coord.parse::<i64>().unwrap();
        }

        self.beacons.push((x, y, z));
    }
}

fn main() {
    let mut day = Day19 {
        scanners: vec![],
    };
    
    for line in aoc::lines("inputs/day_19.txt") {
        if line.starts_with("---") {
            day.scanners.push(Scanner::new());
        } else if !line.is_empty() {
            let last = day.scanners.len() - 1;
            day.scanners[last].add_beacon(&line);
        }
    }

    let mut scanner_locations = vec![];
    match solve_part_1(&day, &mut scanner_locations) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("solve_part_1 failed: {}", reason),
    };

    match solve_part_2(&day, &scanner_locations) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    };
}

fn find_overlap(all_beacons: &mut HashSet<Point>, scanner: &Scanner) -> (bool, Point) {
    let rotations: Vec<Box<dyn Fn(Point) -> Point>> = vec![
        // Positive x
        Box::new(|point: Point| ( point.0,  point.1,  point.2)),
        Box::new(|point: Point| ( point.0, -point.2,  point.1)),
        Box::new(|point: Point| ( point.0, -point.1, -point.2)),
        Box::new(|point: Point| ( point.0,  point.2, -point.1)),

        // Negative X
        Box::new(|point: Point| (-point.0, -point.1,  point.2)),
        Box::new(|point: Point| (-point.0,  point.2,  point.1)),
        Box::new(|point: Point| (-point.0,  point.1, -point.2)),
        Box::new(|point: Point| (-point.0, -point.2, -point.1)),

        // Positive Y
        Box::new(|point: Point| ( point.1,  point.2,  point.0)),
        Box::new(|point: Point| ( point.1, -point.0,  point.2)),
        Box::new(|point: Point| ( point.1, -point.2, -point.0)),
        Box::new(|point: Point| ( point.1,  point.0, -point.2)),

        // Negative Y
        Box::new(|point: Point| (-point.1, -point.2,  point.0)),
        Box::new(|point: Point| (-point.1,  point.0,  point.2)),
        Box::new(|point: Point| (-point.1,  point.2, -point.0)),
        Box::new(|point: Point| (-point.1, -point.0, -point.2)),

        // Positive Z
        Box::new(|point: Point| ( point.2,  point.0,  point.1)),
        Box::new(|point: Point| ( point.2, -point.1,  point.0)),
        Box::new(|point: Point| ( point.2, -point.0, -point.1)),
        Box::new(|point: Point| ( point.2,  point.1, -point.0)),

        // Negative Z
        Box::new(|point: Point| (-point.2, -point.0,  point.1)),
        Box::new(|point: Point| (-point.2,  point.1,  point.0)),
        Box::new(|point: Point| (-point.2,  point.0, -point.1)),
        Box::new(|point: Point| (-point.2, -point.1, -point.0)),
    ];

    for rotation in &rotations {
        let rotation_fn = rotation;

        for known_point in all_beacons.iter() {
            for fixed_point in &scanner.beacons {
                let rotated_point = rotation_fn(*fixed_point);

                let dx = known_point.0 - rotated_point.0;
                let dy = known_point.1 - rotated_point.1;
                let dz = known_point.2 - rotated_point.2;

                let mut matched_beacons = 0;
                for test_point in &scanner.beacons {
                    let p = rotation_fn(*test_point);
                    let q = (p.0 + dx, p.1 + dy, p.2 + dz);

                    if all_beacons.contains(&q) {
                        matched_beacons += 1;
                        if matched_beacons >= 12 {
                            for point in &scanner.beacons {
                                let p = rotation_fn(*point);
                                let q = (p.0 + dx, p.1 + dy, p.2 + dz);
                    
                                all_beacons.insert(q);
                            }

                            return (true, (dx, dy, dz));
                        }
                    }
                }
            }
        }
    }

    return (false, (0, 0 ,0));
}

fn solve_part_1(day: &Day19, scanner_locations: &mut Vec<Point>) -> Result<i64, String> {
    scanner_locations.push((0, 0, 0));

    let mut all_beacons = HashSet::new();
    for beacon in &day.scanners[0].beacons {
        all_beacons.insert(*beacon);
    }

    let mut all_scanners = VecDeque::new();
    for i in 1..day.scanners.len() {
        all_scanners.push_back(&day.scanners[i]);
    }

    while !all_scanners.is_empty() {
        if let Some(scanner) = all_scanners.pop_front() {
            let (found, scanner_location) =
                find_overlap(&mut all_beacons, &scanner);

            if !found {
                all_scanners.push_back(&scanner);
            } else {
                scanner_locations.push(scanner_location);
            }
        }
    }

    return Ok(all_beacons.len() as i64);
}

fn solve_part_2(_day: &Day19, scanner_locations: &Vec<Point>) -> Result<i64, String> {
    let mut max_distance = i64::MIN;
    for i in 0..scanner_locations.len() {
        let first = &scanner_locations[i];

        for j in i..scanner_locations.len() {
            let second = &scanner_locations[j];

            let distance_x = (first.0 - second.0).abs();
            let distance_y = (first.1 - second.1).abs();
            let distance_z = (first.2 - second.2).abs();
            let distance = distance_x + distance_y + distance_z;

            if distance > max_distance {
                max_distance = distance;
            }
        }
    }

    return Ok(max_distance);
}
