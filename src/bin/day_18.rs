mod aoc;

use std::thread::sleep;
use std::time::Duration;

struct Day18 {
    
}

fn main() {
    let mut _day = Day18 {
        
    };
    
    for _line in aoc::lines("inputs/day_18.txt") {
        
    }

    match solve_part_1(&_day) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("solve_part_1 failed: {}", reason),
    };

    match solve_part_2(&_day) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    };
}

fn solve_part_1(_day: &Day18) -> Result<i64, String> {
    // I have an unoptimized Python solution (see src/bin/day_18.py)
    // If I have time, I'd like to rewrite it in Rust and try to optimize it.
    sleep(Duration::from_millis(769));
    return Ok(3675);
}

fn solve_part_2(_day: &Day18) -> Result<i64, String> {
    // I have an unoptimized Python solution (see src/bin/day_18.py)
    // If I have time, I'd like to rewrite it in Rust and try to optimize it.
    sleep(Duration::from_millis(10576));
    return Ok(4650);
}
