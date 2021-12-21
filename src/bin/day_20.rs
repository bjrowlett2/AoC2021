mod aoc;

use std::collections::HashMap;

type Point = (i64, i64);

struct Day20 {
    input_image: HashMap<Point, char>,
    image_enhancement_algorithm: Vec<char>,
}

fn main() {
    let mut day = Day20 {
        input_image: HashMap::new(),
        image_enhancement_algorithm: vec![],
    };

    let lines = aoc::lines("inputs/day_20.txt");

    let algorithm = lines[0].chars();
    day.image_enhancement_algorithm = algorithm.collect();

    for i in 2..lines.len() {
        let mut x = 0;
        let y = (i - 2) as i64;
        for ch in lines[i].chars() {
            day.input_image.insert((x, y), ch);
            x += 1;
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

fn enhance(image: &HashMap<Point, char>, x: i64, y: i64, outside: char) -> usize {
    let mut index = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            let mut ch = outside;
            let point = (x + dx, y + dy);
            if let Some(pixel) = image.get(&point) {
                ch = *pixel;
            }
            
            index *= 2;
            if ch == '#' {
                index += 1;
            }
        }
    }

    return index;
}

fn solve_part_1(day: &Day20) -> Result<i64, String> {
    let mut outside = '.';
    let outside_swaps = vec![
        day.image_enhancement_algorithm[0],
        day.image_enhancement_algorithm[511],
    ];
    
    let mut input_image = day.input_image.clone();
    for step in 0..2 {
        let mut output_image = HashMap::new();

        let border = 1;
        let min_x = input_image.keys().min_by_key(|p| p.0).unwrap().0;
        let max_x = input_image.keys().max_by_key(|p| p.0).unwrap().0;
        let min_y = input_image.keys().min_by_key(|p| p.1).unwrap().1;
        let max_y = input_image.keys().max_by_key(|p| p.1).unwrap().1;
        for y in (min_y - border)..=(max_y + border) {
            for x in (min_x - border)..=(max_x + border) {
                let index = enhance(&input_image, x, y, outside);
                output_image.insert((x, y), day.image_enhancement_algorithm[index]);
            }
        }

        outside = outside_swaps[step % 2];

        input_image = output_image.clone();
    }

    let mut pixels_lit = 0;
    for pixel in input_image.values() {
        if *pixel == '#' {
            pixels_lit += 1;
        }
    }

    return Ok(pixels_lit);
}

fn solve_part_2(day: &Day20) -> Result<i64, String> {
    let mut outside = '.';
    let outside_swaps = vec![
        day.image_enhancement_algorithm[0],
        day.image_enhancement_algorithm[511],
    ];
    
    let mut input_image = day.input_image.clone();
    for step in 0..50 {
        let mut output_image = HashMap::new();

        let border = 1;
        let min_x = input_image.keys().min_by_key(|p| p.0).unwrap().0;
        let max_x = input_image.keys().max_by_key(|p| p.0).unwrap().0;
        let min_y = input_image.keys().min_by_key(|p| p.1).unwrap().1;
        let max_y = input_image.keys().max_by_key(|p| p.1).unwrap().1;
        for y in (min_y - border)..=(max_y + border) {
            for x in (min_x - border)..=(max_x + border) {
                let index = enhance(&input_image, x, y, outside);
                output_image.insert((x, y), day.image_enhancement_algorithm[index]);
            }
        }

        outside = outside_swaps[step % 2];

        input_image = output_image.clone();
    }

    let mut pixels_lit = 0;
    for pixel in input_image.values() {
        if *pixel == '#' {
            pixels_lit += 1;
        }
    }

    return Ok(pixels_lit);
}
