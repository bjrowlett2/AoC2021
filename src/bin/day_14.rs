mod aoc;

use std::collections::HashMap;

#[derive(Clone)]
struct Day14 {
    polymer_template: String,
    pair_insertion_rules: HashMap<String, String>,
}

fn main() {
    let mut day = Day14 {
        polymer_template: String::new(),
        pair_insertion_rules: HashMap::new(),
    };

    for line in aoc::lines("inputs/day_14.txt") {
        if day.polymer_template.is_empty() {
            day.polymer_template = line.to_string();
        } else if line.contains(" -> ") {
            let mut parts = line.split(" -> ");

            let key = match parts.next() {
                Some(value) => value.to_string(),
                None => panic!("No pair insertion key found"),
            };

            let value = match parts.next() {
                Some(value) => value.to_string(),
                None => panic!("No pair insertion value found"),
            };
    
            day.pair_insertion_rules.insert(key, value);
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

trait MinMax<T> {
    fn min(&self) -> T;
    fn max(&self) -> T;
    fn max_minus_min(&self) -> T;
}

impl MinMax<i64> for HashMap<String, i64> {
    fn min(&self) -> i64 {
        let values = self.values();
        if let Some(minimum) = values.min() {
            return *minimum;
        }

        return 0;
    }

    fn max(&self) -> i64 {
        let values = self.values();
        if let Some(maximum) = values.max() {
            return *maximum;
        }

        return 0;
    }

    fn max_minus_min(&self) -> i64 {
        return self.max() - self.min();
    }
}

fn apply_step(day: &mut Day14) {
    let mut insertions = vec![];
    for i in 0..day.polymer_template.len() - 1 {
        let slice = &day.polymer_template[i..];
        for pair_insertion_rule in &day.pair_insertion_rules {
            if slice.starts_with(pair_insertion_rule.0) {
                insertions.push((i + 1, pair_insertion_rule.1));
            }
        }
    }

    insertions.sort_by_key(|x| -(x.0 as i64));

    for insertion in insertions {
        day.polymer_template.insert_str(insertion.0, insertion.1);
    }
}

fn solve_part_1(day: &Day14) -> Result<i64, String> {
    let mut clone = day.clone();

    for _ in 0..10 {
        apply_step(&mut clone);
    }

    let mut quantities = HashMap::new();
    for ch in clone.polymer_template.chars() {
        let key = ch.to_string();
        quantities.increment_by(&key, 1);
    }

    return Ok(quantities.max_minus_min());
}

trait ModifyBy<K, V> {
    fn decrement_by(&mut self, key: &K, amount: V);
    fn increment_by(&mut self, key: &K, amount: V);
}

impl ModifyBy<String, i64> for HashMap<String, i64> {
    fn decrement_by(&mut self, key: &String, amount: i64) {
        self.increment_by(key, -amount);
    }

    fn increment_by(&mut self, key: &String, amount: i64) {
        let entry = self.entry(key.to_string());
        let value = entry.or_insert(0);
        *value += amount;
    }
}

fn make_key(s1: &String, s2: &String) -> String {
    return s1.to_owned() + s2;
}

fn apply_step_fast(histogram: &mut HashMap<String, i64>, day: &Day14) {
    let mut deletions = HashMap::new();
    let mut insertions = HashMap::new();

    for pair_insertion_rule in &day.pair_insertion_rules {
        let key = pair_insertion_rule.0.to_string();

        if histogram.contains_key(&key) {
            let prefix = key[0..1].to_string();
            let suffix = key[1..2].to_string();

            let new_key_1 = make_key(&prefix, pair_insertion_rule.1);
            let new_key_2 = make_key(pair_insertion_rule.1, &suffix);

            let current_value = *histogram.get(&key).unwrap();

            deletions.increment_by(&key, current_value);
            insertions.increment_by(&new_key_1, current_value);
            insertions.increment_by(&new_key_2, current_value);
        }
    }

    for deletion in &deletions {
        histogram.decrement_by(deletion.0, *deletion.1);
    }

    for insertion in &insertions {
        histogram.increment_by(insertion.0, *insertion.1);
    }
}

fn solve_part_2(day: &Day14) -> Result<i64, String> {
    let mut histogram = HashMap::new();

    // Initialize the histogram with all pairs
    // of characters from the given polymer_template.
    for i in 0..day.polymer_template.len() - 1 {
        let key = &day.polymer_template[i..i+2];
        histogram.increment_by(&key.to_string(), 1);
    }

    for _ in 0..40 {
        apply_step_fast(&mut histogram, &day);
    }

    let mut quantities = HashMap::from([
        (day.polymer_template[0..1].to_string(), 1)
    ]);

    for result in &histogram {
        let key = result.0[1..2].to_string();
        quantities.increment_by(&key, *result.1);
    }

    return Ok(quantities.max_minus_min());
}
