mod aoc;

use std::collections::HashMap;
use std::collections::HashSet;

type CaveSet = HashSet<String>;
type CaveMap = HashMap<String, CaveSet>;

struct Day12 {
    caves: CaveMap,
}

trait IsLowercase {
    fn is_lowercase(&self) -> bool;
}

impl IsLowercase for String {
    fn is_lowercase(&self) -> bool {
        return self.chars().all(|c| c.is_lowercase());
    }
}

fn main() {
    let mut day = Day12 {
        caves: CaveMap::new(),
    };

    for line in aoc::lines("inputs/day_12.txt") {
        let (src, dest) = parse_line(&line);
        connect_path(&mut day.caves, &src, &dest);
        connect_path(&mut day.caves, &dest, &src);
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

fn parse_line(line: &String) -> (String, String) {
    let mut parts = line.split("-");

    let src = match parts.next() {
        Some(value) => value.to_string(),
        None => panic!("No source cave found"),
    };

    let dest = match parts.next() {
        Some(value) => value.to_string(),
        None => panic!("No destination cave found"),
    };

    return (src, dest);
}

fn connect_path(caves: &mut CaveMap, src: &String, dest: &String) {
    if !caves.contains_key(src) {
        caves.insert(src.to_string(), CaveSet::new());
    }

    if let Some(value) = caves.get_mut(src) {
        value.insert(dest.to_string());
    }
}

fn navigate(caves: &CaveMap, part_1: bool) -> i64 {
    let mut visited = vec![];
    let start = String::from("start");
    return traverse(&start, &mut visited, caves, part_1);
}

fn traverse(at: &String, visited: &mut Vec<String>, caves: &CaveMap, was_duplicate: bool) -> i64 {
    if at == "end" {
        return 1;
    }

    let mut is_duplicate = was_duplicate;
    if at.is_lowercase() {
        if visited.contains(&at) {
            is_duplicate = true;
        }
    }

    let mut paths = 0;
    if let Some(neighbors) = caves.get(&at.to_string()) {
        for next in neighbors {
            if next == "start" {
                continue;
            }

            if is_duplicate {
                if next.is_lowercase() {
                    if visited.contains(&next) {
                        continue;
                    }
                }
            }
    
            if at.is_lowercase() {
                visited.push(at.to_string());
            }
    
            paths += traverse(&next, visited, caves, is_duplicate);
    
            if at.is_lowercase() {
                visited.pop();
            }
        }
    }

    return paths;
}

fn solve_part_1(day: &Day12) -> Result<i64, String> {
    return Ok(navigate(&day.caves, true));
}

fn solve_part_2(day: &Day12) -> Result<i64, String> {
    return Ok(navigate(&day.caves, false));
}
