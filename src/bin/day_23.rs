mod aoc;

use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::{min, max};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

struct Day23 {
    burrow: Burrow,
}

type Point = (usize, usize);

#[derive(Clone, Eq, PartialEq)]
struct Amphipod {
    letter: char,
    location: Point,
}

#[derive(Clone)]
struct Burrow {
    width: usize,
    height: usize,
    layout: Vec<Vec<char>>,
    amphipods: Vec<Amphipod>,
}

impl Burrow {
    fn new() -> Burrow {
        return Burrow {
            width: 0,
            height: 0,
            layout: vec![],
            amphipods: vec![],
        }
    }
}

#[derive(Eq, PartialEq)]
struct StateWithCost {
    cost: i64,
    amphipods: Vec<Amphipod>,
}

impl Ord for StateWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.cost.cmp(&self.cost);
    }
}

impl PartialOrd for StateWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn load(file: &str) -> Day23 {
    let mut day = Day23 {
        burrow: Burrow::new(),
    };

    for line in aoc::lines(file) {
        day.burrow.height += 1;
        if day.burrow.width == 0 {
            day.burrow.width = line.len();
        }

        day.burrow.layout.push(vec![]);

        let last = day.burrow.height - 1;

        let chars = line.chars();
        for (x, ch) in chars.enumerate() {
            if ch.is_alphabetic() {
                day.burrow.layout[last].push('.');
                day.burrow.amphipods.push(Amphipod {
                    letter: ch,
                    location: (x, last),
                });
            } else {
                day.burrow.layout[last].push(ch);
            }
        }

        while day.burrow.layout[last].len() < day.burrow.width {
            day.burrow.layout[last].push(' ');
        }
    }

    return day;
}

fn main() {
    let day_1 = load("inputs/day_23_1.txt");
    let day_2 = load("inputs/day_23_2.txt");

    match solve_part_1(&day_1) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("solve_part_1 failed: {}", reason),
    }

    match solve_part_2(&day_2) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    }
}

fn is_complete(amphipods: &Vec<Amphipod>) -> bool {
    for amphipod in amphipods.iter() {
        // @Robustness: Assume other rules are in place to prevent
        // an amphipod from being in the hallway in front of a room.

        if amphipod.letter == 'A' {
            if amphipod.location.0 != 3 {
                return false;
            }
        } else if amphipod.letter == 'B' {
            if amphipod.location.0 != 5 {
                return false;
            }
        } else if amphipod.letter == 'C' {
            if amphipod.location.0 != 7 {
                return false;
            }
        } else if amphipod.letter == 'D' {
            if amphipod.location.0 != 9 {
                return false;
            }
        }
    }

    return true;
}

fn can_move(amphipods: &Vec<Amphipod>, this: &Amphipod, x: usize, y: usize, part: i64) -> Option<i64> {
    // Rule:
    //  Amphipods will never stop on the space immediately outside any room.
    //  Handled implicitly, by never asking can_move(...) on those coordinates.

    // Rule:
    //  Amphipods will never move from the hallway into a room unless that room
    //  is their destination room and that room contains no amphipods which do
    //  not also have that room as their own destination.

    for amphipod in amphipods.iter() {
        if amphipod.letter != this.letter {
            if amphipod.location == (x, y + 1) {
                return None;
            }
            
            if part == 2 {
                if amphipod.location == (x, y + 2) {
                    return None;
                } else if amphipod.location == (x, y + 3) {
                    return None;
                }
            }
        }
    }

    // Rule:
    //  Once an amphipod stops moving in the hallway, it will stay in that spot
    //  until it can move into a room.

    if (y == 1) && (this.location.1 == 1) {
        return None;
    }

    for amphipod in amphipods.iter() {
        if amphipod.location == (x, y) {
            return None; // Destination already occupied
        }
    }

    for yt in 1..this.location.1 {
        for amphipod in amphipods.iter() {
            if amphipod.location == (this.location.0, yt) {
                return None; // Path from source to Hallway blocked
            }
        }
    }

    let min_x = min(x, this.location.0);
    let max_x = max(x, this.location.0);
    for xt in min_x..=max_x {
        if xt == this.location.0 {
            continue;
        }
        
        for amphipod in amphipods.iter() {
            if amphipod.location == (xt, 1) {
                return None; // Path through the Hallway blocked
            }
        }
    }

    for yt in 1..y {
        for amphipod in amphipods.iter() {
            if amphipod.location == (x, yt) {
                return None; // Path from Hallway to destination blocked
            }
        }
    }

    let to_hallway = (this.location.1 - 1) as i64;
    let to_correct_x = ((this.location.0 as i64) - (x as i64)).abs();
    let from_hallway = (y - 1) as i64;
    
    let factor = 10_i64.pow((this.letter as u32) - ('A' as u32));
    return Some(factor * (to_hallway + to_correct_x + from_hallway));
}

fn is_in_correct_place(amphipods: &Vec<Amphipod>, this: &Amphipod, part: i64) -> bool {
    if this.letter == 'A' {
        if this.location.0 != 3 {
            return false;
        }
    } else if this.letter == 'B' {
        if this.location.0 != 5 {
            return false;
        }
    } else if this.letter == 'C' {
        if this.location.0 != 7 {
            return false;
        }
    } else if this.letter == 'D' {
        if this.location.0 != 9 {
            return false;
        }
    }

    let mut last_y = 3;
    if part == 2 { last_y = 5; }

    let x = this.location.0;
    let y = this.location.1;
    if (2 <= y) && (y < last_y) {
        let mut amphipods_below_i = vec![];
        let mut has_amphipods_below = false;
        for i in 0..amphipods.len() {
            if amphipods[i].location == (x, y + 1) {
                amphipods_below_i.push(i);
                has_amphipods_below = true;
            }
            
            if part == 2 {
                if amphipods[i].location == (x, y + 2) {
                    amphipods_below_i.push(i);
                    has_amphipods_below = true;
                } else if amphipods[i].location == (x, y + 3) {
                    amphipods_below_i.push(i);
                    has_amphipods_below = true;
                }
            }
        }

        if !has_amphipods_below {
            return false;
        }
    
        if has_amphipods_below {
            for i in &amphipods_below_i {
                let amphipod_below = &amphipods[*i];
                if amphipod_below.letter != this.letter {
                    return false;
                }
            }
        }
    }

    return true;
}

fn encode(amphipods: &Vec<Amphipod>) -> [u8; 23] {
    let mut encoding = [0; 23];
    let index_mapping = HashMap::from([
        (( 1, 1),  0), (( 2, 1),  1),
        (( 4, 1),  2), (( 6, 1),  3), (( 8, 1),  4),
        ((10, 1),  5), ((11, 6),  6),

        (( 3, 2),  7), (( 3, 3),  8), (( 3, 4),  9), (( 3, 5), 10),
        (( 5, 2), 11), (( 5, 3), 12), (( 5, 4), 13), (( 5, 5), 14),
        (( 7, 2), 15), (( 7, 3), 16), (( 7, 4), 17), (( 7, 5), 18),
        (( 9, 2), 19), (( 9, 3), 20), (( 9, 4), 21), (( 9, 5), 22),
    ]);

    for amphipod in amphipods.iter() {
        if let Some(index) = index_mapping.get(&amphipod.location) {
            let mut value = 0;
            if amphipod.letter == 'A' {
                value = 1;
            } else if amphipod.letter == 'B' {
                value = 2;
            } else if amphipod.letter == 'C' {
                value = 3;
            } else if amphipod.letter == 'D' {
                value = 4;
            }

            encoding[*index as usize] = value;
        }
    }

    return encoding;
}

fn djikstra(day: &Day23, part: i64) -> Result<i64, String> {
    let mut seen = HashSet::new();
    let mut states = BinaryHeap::new();
    states.push(StateWithCost {
        cost: 0,
        amphipods: day.burrow.amphipods.clone(),
    });

    while let Some(state) = states.pop() {
        let encoding = encode(&state.amphipods);
        if !seen.contains(&encoding) {
            seen.insert(encoding);

            if is_complete(&state.amphipods) {
                return Ok(state.cost)
            } else {
                for i in 0..state.amphipods.len() {
                    let amphipod = &state.amphipods[i];
                    if is_in_correct_place(&state.amphipods, &amphipod, part) {
                        continue;
                    }
    
                    let mut moves_to_check = vec![
                        ( 1, 1), ( 2, 1),
                        ( 4, 1), ( 6, 1), ( 8, 1),
                        (10, 1), (11, 1),
                    ];
    
                    if amphipod.letter == 'A' {
                        moves_to_check.push((3, 2));
                        moves_to_check.push((3, 3));

                        if part == 2 {
                            moves_to_check.push((3, 4));
                            moves_to_check.push((3, 5));
                        }
                    } else if amphipod.letter == 'B' {
                        moves_to_check.push((5, 2));
                        moves_to_check.push((5, 3));

                        if part == 2 {
                            moves_to_check.push((5, 4));
                            moves_to_check.push((5, 5));
                        }
                    }  else if amphipod.letter == 'C' {
                        moves_to_check.push((7, 2));
                        moves_to_check.push((7, 3));

                        if part == 2 {
                            moves_to_check.push((7, 4));
                            moves_to_check.push((7, 5));
                        }
                    } else if amphipod.letter == 'D' {
                        moves_to_check.push((9, 2));
                        moves_to_check.push((9, 3));

                        if part == 2 {
                            moves_to_check.push((9, 4));
                            moves_to_check.push((9, 5));
                        }
                    }

                    for (x, y) in moves_to_check.iter() {
                        if let Some(energy) = can_move(&state.amphipods, &amphipod, *x, *y, part) {
                            let new_cost = state.cost + energy;
                            let mut new_state = state.amphipods.clone();

                            new_state[i].location = (*x, *y);
                            let new_encoding = encode(&new_state);
    
                            if !seen.contains(&new_encoding) {
                                states.push(StateWithCost {
                                    cost: new_cost,
                                    amphipods: new_state
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    return Err(String::from("No solution found"));
}

fn solve_part_1(day: &Day23) -> Result<i64, String> {
    return djikstra(day, 1);
}

fn solve_part_2(day: &Day23) -> Result<i64, String> {
    return djikstra(day, 2);
}
