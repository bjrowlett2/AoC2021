mod aoc;

struct Day03 {
    bits: usize,
    numbers: Vec<i64>,
}

fn main() {
    let mut day = Day03 {
        bits: 0,
        numbers: vec![],
    };

    for line in aoc::lines("inputs/day_03.txt") {
        if line.len() > day.bits {
            day.bits = line.len();
        }

        match i64::from_str_radix(&line, 2) {
            Ok(value) => day.numbers.push(value),
            Err(reason) => panic!("i64::from_str_radix failed: {}", reason),
        };
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

fn is_set(number: &i64, place: usize) -> bool {
    return ((number >> place) & 1) == 1;
}

fn most_common(numbers: &Vec<i64>, place: usize) -> i64 {
    let mut num_ones = 0;
    let mut num_zeros = 0;

    for number in numbers {
        match is_set(number, place) {
            true => num_ones += 1,
            false => num_zeros += 1,
        };
    }

    match num_zeros > num_ones {
        true => return 0,

        // If 0 and 1 are equally common, keep
        // values with a 1 in the position being considered.
        false => return 1,
    };
}

fn least_common(numbers: &Vec<i64>, place: usize) -> i64 {
    let mut num_ones = 0;
    let mut num_zeros = 0;

    for number in numbers {
        match is_set(number, place) {
            true => num_ones += 1,
            false => num_zeros += 1,
        };
    }

    match num_ones < num_zeros {
        true => return 1,

        // If 0 and 1 are equally common, keep
        // values with a 0 in the position being considered.
        false => return 0,
    };
}

fn solve_part_1(day: &Day03) -> Result<i64, String> {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for bit in 0..day.bits {
        let place = day.bits - bit - 1;

        gamma_rate <<= 1;
        gamma_rate |= most_common(&day.numbers, place);

        epsilon_rate <<= 1;
        epsilon_rate |= least_common(&day.numbers, place);
    }

    return Ok(gamma_rate * epsilon_rate);
}

fn solve_part_2(day: &Day03) -> Result<i64, String> {
    let mut oxygen = day.numbers.to_vec();

    for bit in 0..day.bits {
        let place = day.bits - bit - 1;

        let most_common = most_common(&oxygen, place);

        oxygen.retain(|number| {
            return ((number >> place) & 1) == most_common;
        });

        if oxygen.len() == 1 {
            break;
        }
    }

    let mut carbon_dioxide = day.numbers.to_vec();

    for bit in 0..day.bits {
        let place = day.bits - bit - 1;

        let least_common = least_common(&carbon_dioxide, place);

        carbon_dioxide.retain(|number| {
            return ((number >> place) & 1) == least_common;
        });
        
        if carbon_dioxide.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating = oxygen[0];
    let carbon_dioxide_scrubber_rating = carbon_dioxide[0];
    return Ok(oxygen_generator_rating * carbon_dioxide_scrubber_rating);
}
