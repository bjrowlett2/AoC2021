mod aoc;

use std::hash::Hash;
use std::hash::Hasher;
use std::collections::HashMap;

struct Day21 {
    players: Vec<Player>,
}

#[derive(Clone)]
struct Player {
    score: i64,
    position: i64,
}

fn main() {
    let initial_player = Player {
        score: 0,
        position: 0,
    };

    let mut day = Day21 {
        players: vec![initial_player; 2],
    };

    for line in aoc::lines("inputs/day_21.txt") {
        if let Some(rest) = line.strip_prefix("Player 1 starting position: ") {
            if let Ok(position) = rest.parse::<i64>() {
                day.players[0].position = position;
            }
        } else if let Some(rest) = line.strip_prefix("Player 2 starting position: ") {
            if let Ok(position) = rest.parse::<i64>() {
                day.players[1].position = position;
            }
        }
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

fn solve_part_1(day: &Day21) -> Result<i64, String> {
    let mut turns = 0;
    let mut deterministic_dice = 1;
    let mut players = day.players.clone();
    loop {
        let index = turns % 2;
        let moves = (3 * deterministic_dice) + 3;

        players[index].position += moves;
        while players[index].position > 10 {
            players[index].position -= 10;
        }

        players[index].score += players[index].position;

        turns += 1;
        deterministic_dice += 3;
        while deterministic_dice > 100 {
            deterministic_dice -= 100;
        }

        if players[index].score >= 1000 {
            break;
        }
    }

    let mut score = 0;
    let rolls = (turns as i64) * 3;
    if players[0].score >= 1000 {
        score = players[1].score * rolls;
    } else if players[1].score >= 1000 {
        score = players[0].score * rolls;
    }

    return Ok(score);
}

#[derive(Clone)]
struct State {
    scores: [i64; 2],
    positions: [i64; 2],
}

impl State {
    fn from(day: &Day21) -> State {
        return State {
            scores: [0; 2],
            positions: [
                day.players[0].position,
                day.players[1].position
            ],
        };
    }
}

impl Eq for State { }

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        return self.scores[0] == other.scores[0]
            && self.scores[1] == other.scores[1]
            && self.positions[0] == other.positions[0]
            && self.positions[1] == other.positions[1];
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.scores[0].hash(state);
        self.scores[1].hash(state);
        self.positions[0].hash(state);
        self.positions[1].hash(state);
    }
}

trait IncrementBy<K, V> {
    fn increment_by(&mut self, key: &K, amount: V);
}

impl IncrementBy<State, i64> for HashMap<State, i64> {
    fn increment_by(&mut self, key: &State, amount: i64) {
        let entry = self.entry(key.clone());
        let value = entry.or_insert(0);
        *value += amount;
    }
}

fn num_copies(dirac_dice: i64) -> i64 {
    let copies = HashMap::from([
        (3, 1), (4, 3), (5, 6),
        (6, 7), (7, 6), (8, 3), (9, 1),
    ]);

    return match copies.get(&dirac_dice) {
        Some(value) => *value,
        None => panic!("Invalid dirac dice roll"),
    };
}

fn solve_part_2(day: &Day21) -> Result<i64, String> {
    let mut wins = [0; 2];
    let mut universes = HashMap::from([
        (State::from(day), 1),
    ]);

    while !universes.is_empty() {
        for player in 0..=1 {
            let mut new_universes = HashMap::new();
            for dirac_dice in 3..=9 {
                let copies = num_copies(dirac_dice);
    
                for universe in universes.iter() {
                    let count = universe.1 * copies;
                    let mut state = universe.0.clone();

                    state.positions[player] += dirac_dice;
                    while state.positions[player] > 10 {
                        state.positions[player] -= 10;
                    }

                    state.scores[player] += state.positions[player];
                    if state.scores[player] >= 21 {
                        wins[player] += count;
                    } else {
                        new_universes.increment_by(&state, count);
                    }
                }
            }

            universes = new_universes;
        }
    }

    wins.sort();
    return Ok(wins[1]);
}
