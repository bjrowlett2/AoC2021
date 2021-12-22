mod aoc;

type Point = (i64, i64);

#[derive(Clone)]
struct Day11 {
    width: i64,
    height: i64,
    octopuses: Vec<i64>,
}

impl Day11 {
    fn new() -> Day11 {
        return Day11 {
            width: 0,
            height: 0,
            octopuses: vec![],
        };
    }

    fn reset(&mut self, point: Point) {
        let (x, y) = point;
        let index = y * self.width + x;
        self.octopuses[index as usize] = 0;
    }
    
    fn increase(&mut self, point: Point) {
        let (x, y) = point;
        let index = y * self.width + x;
        self.octopuses[index as usize] += 1;
    }

    fn flashes(&self, point: Point) -> bool {
        let (x, y) = point;
        let index = y * self.width + x;

        // Check if equal to 10 so that each
        // octopus can only flash once per step.
        return self.octopuses[index as usize] == 10;
    }

    fn neighbors(&self, point: Point) -> Vec<Point> {
        let (x, y) = point;
        let mut points = vec![
            (x - 1, y - 1), (x + 0, y - 1), (x + 1, y - 1),
            (x - 1, y + 0),                 (x + 1, y + 0),
            (x - 1, y + 1), (x + 0, y + 1), (x + 1, y + 1),
        ];

        points.retain(|value| {
            return (0..self.width).contains(&value.0)
                && (0..self.height).contains(&value.1);
        });

        return points;
    }

    fn step(&mut self) -> i64 {
        let mut flashes = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let octopus = (x, y);
                self.increase(octopus);
    
                if self.flashes(octopus) {
                    flashes.push(octopus);
                }
            }
        }
    
        let mut reset = vec![];
        while !flashes.is_empty() {
            let octopus = match flashes.pop() {
                Some(value) => value,
                None => panic!("Tried to pop, but no value found"),
            };
    
            for neighbor in self.neighbors(octopus) {
                self.increase(neighbor);
    
                if self.flashes(neighbor) {
                    flashes.push(neighbor);
                }
            }
    
            reset.push(octopus);
        }
    
        let mut flashes = 0;
        for octopus in &reset {
            flashes += 1;
    
            self.reset(*octopus);
        }
    
        return flashes;
    }
}

fn main() {
    let mut day = Day11::new();

    for line in aoc::lines("inputs/day_11.txt") {
        day.height += 1;

        if day.width == 0 {
            day.width = line.len() as i64;
        }

        for energy in line.chars() {
            let s = String::from(energy);
            let value = match s.parse::<i64>() {
                Ok(value) => value,
                Err(reason) => panic!("String::parse failed: {}", reason),
            };
            
            day.octopuses.push(value);
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

fn solve_part_1(day: &Day11) -> Result<i64, String> {
    let mut clone = day.clone();

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += clone.step();
    }

    return Ok(flashes);
}

fn solve_part_2(day: &Day11) -> Result<i64, String> {
    let mut clone = day.clone();

    let mut steps = 1;
    while clone.step() != 100 {
        steps += 1;
    }

    return Ok(steps);
}
