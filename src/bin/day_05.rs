mod aoc;

use std::cmp;
use std::ops::Range;

type Point = (i64, i64);

struct Line {
    p0: Point,
    p1: Point,
}

impl Line {
    fn new(start: Point, finish: Point) -> Line {
        return Line {
            p0: start,
            p1: finish,
        };
    }

    fn slope(&self) -> (i64, i64) {
        let mut slope_x = 0;
        if self.p0.0 < self.p1.0 {
            slope_x = 1;
        } else if self.p0.0 > self.p1.0 {
            slope_x = -1;
        }

        let mut slope_y = 0;
        if self.p0.1 < self.p1.1 {
            slope_y = 1;
        } else if self.p0.1 > self.p1.1 {
            slope_y = -1;
        }

        return (slope_x, slope_y);
    }

    fn is_vertical(&self) -> bool {
        return self.p0.0 == self.p1.0;
    }

    fn vertical_bounds(&self) -> Range<i64> {
        return Range {
            start: cmp::min(self.p0.1, self.p1.1),
            end:   cmp::max(self.p0.1, self.p1.1) + 1,
        };
    }

    fn is_horizontal(&self) -> bool {
        return self.p0.1 == self.p1.1;
    }

    fn horizontal_bounds(&self) -> Range<i64> {
        return Range {
            start: cmp::min(self.p0.0, self.p1.0),
            end:   cmp::max(self.p0.0, self.p1.0) + 1,
        };
    }
}

struct Day05 {
    lines: Vec<Line>,
}

fn main() {
    let mut day = Day05 {
        lines: vec![],
    };

    for line in aoc::lines("inputs/day_05.txt") {
        let mut coordinates = line.split(" -> ");

        let start = match coordinates.next() {
            Some(value) => parse_coordinate(value),
            None => panic!("No start coordinate found"),
        };

        let finish = match coordinates.next() {
            Some(value) => parse_coordinate(value),
            None => panic!("No finish coordinate found"),
        };

        day.lines.push(Line::new(start, finish));
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

fn parse_value(value: &str) -> i64 {
    return match value.parse::<i64>() {
        Ok(value) => value,
        Err(reason) => panic!("String::parse failed: {}", reason),
    };
}

fn parse_coordinate(coordinate: &str) -> Point {
    let mut values = coordinate.split(",");

    let x = match values.next() {
        Some(value) => parse_value(value),
        None => panic!("parse_coordinate failed: No x coordinate found"),
    };

    let y = match values.next() {
        Some(value) => parse_value(value),
        None => panic!("parse_coordinate failed: No y coordinate found"),
    };

    return (x, y);
}

struct Map {
    width: usize,
    points: Vec<i64>,
}

impl Map {
    fn new() -> Map {
        let size = 1000;
        return Map {
            width: size,
            points: vec![0; size * size],
        };
    }

    fn mark(&mut self, x: i64, y: i64) {
        let p = (x as usize, y as usize);
        self.points[p.0 + (p.1 * self.width)] += 1;
    }

    fn count_dangerous_areas(&self) -> i64 {
        let mut dangerous_areas = 0;
        for i in 0..self.points.len() {
            if self.points[i] > 1 {
                dangerous_areas += 1;
            }
        }

        return dangerous_areas;
    }
}

fn solve_part_1(day: &Day05) -> Result<i64, String> {
    let mut map = Map::new();

    for line in &day.lines {
        if line.is_vertical() {
            for y in line.vertical_bounds() {
                map.mark(line.p0.0, y);
            }
        } else if line.is_horizontal() {
            for x in line.horizontal_bounds() {
                map.mark(x, line.p0.1);
            }
        }
    }

    return Ok(map.count_dangerous_areas());
}

fn solve_part_2(day: &Day05) -> Result<i64, String> {
    let mut map = Map::new();

    for line in &day.lines {
        if line.is_vertical() {
            for y in line.vertical_bounds() {
                map.mark(line.p0.0, y);
            }
        } else if line.is_horizontal() {
            for x in line.horizontal_bounds() {
                map.mark(x, line.p0.1);
            }
        } else {
            let mut p = line.p0;
            let slope = line.slope();

            while p.0 != (line.p1.0 + slope.0) {
                map.mark(p.0, p.1);

                p.0 += slope.0;
                p.1 += slope.1;
            }
        }
    }

    return Ok(map.count_dangerous_areas());
}
