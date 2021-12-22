mod aoc;

use std::collections::VecDeque;

type Point = (i64, i64);

struct Heightmap {
    width: i64,
    height: i64,
    elevations: Vec<i64>,
}

impl Heightmap {
    fn new() -> Heightmap {
        return Heightmap {
            width: 0,
            height: 0,
            elevations: vec![],
        };
    }

    fn at(&self, x: i64, y: i64) -> i64 {
        let index = y * self.width + x;
        return self.elevations[index as usize];
    }

    fn is_lower_than(&self, point_1: Point, point_2: Point) -> bool {
        let index_1 = point_1.1 * self.width + point_1.0;
        let index_2 = point_2.1 * self.width + point_2.0;
        return self.elevations[index_1 as usize] < self.elevations[index_2 as usize];
    }

    fn neighbors(&self, x: i64, y: i64) -> Vec<Point> {
        let mut neighbors = vec![];

        if x >= 1 {
            neighbors.push((x - 1, y));
        }

        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }

        if y >= 1 {
            neighbors.push((x, y - 1));
        }

        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }

        return neighbors;
    }

    fn is_low_point(&self, x: i64, y: i64) -> bool {
        for neighbor in self.neighbors(x, y) {
            if !self.is_lower_than((x, y), neighbor) {
                return false;
            }
        }

        return true;
    }

    fn flood_fill(&self, x: i64, y: i64) -> i64 {
        let mut interior = vec![];
        let mut boundary = VecDeque::from([(x, y)]);

        while !boundary.is_empty() {
            let cell = match boundary.pop_front() {
                Some(value) => value,
                None => panic!("No boundary cell found"),
            };

            interior.push(cell);
            for neighbor in self.neighbors(cell.0, cell.1) {
                if interior.contains(&neighbor) {
                    continue;
                }
                
                if boundary.contains(&neighbor) {
                    continue;
                }

                if self.is_lower_than(cell, neighbor) {
                    if self.at(neighbor.0, neighbor.1) < 9 {
                        boundary.push_back(neighbor);
                    }
                }
            }
        }

        return interior.len() as i64;
    }
}

struct Day09 {
    heightmap: Heightmap,
}

fn main() {
    let mut day = Day09 {
        heightmap: Heightmap::new(),
    };

    for line in aoc::lines("inputs/day_09.txt") {
        day.heightmap.height += 1;

        if day.heightmap.width == 0 {
            day.heightmap.width = line.len() as i64;
        }

        for height in line.chars() {
            let s = String::from(height);
            let value = match s.parse::<i64>() {
                Ok(value) => value,
                Err(reason) => panic!("String::parse failed: {}", reason),
            };
            
            day.heightmap.elevations.push(value);
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

fn solve_part_1(day: &Day09) -> Result<i64, String> {
    let mut risk_level = 0;
    for x in 0..day.heightmap.width {
        for y in 0..day.heightmap.height {
            if day.heightmap.is_low_point(x, y) {
                risk_level += day.heightmap.at(x, y) + 1;
            }
        }
    }

    return Ok(risk_level);
}

fn solve_part_2(day: &Day09) -> Result<i64, String> {
    let mut basins = vec![];
    for x in 0..day.heightmap.width {
        for y in 0..day.heightmap.height {
            if day.heightmap.is_low_point(x, y) {
                basins.push(day.heightmap.flood_fill(x, y));
            }
        }
    }

    basins.sort();

    let n = basins.len() - 1;
    return Ok(basins[n] * basins[n - 1] * basins[n - 2]);
}
