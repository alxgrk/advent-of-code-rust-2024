use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let array = parse_input(input);

    let mut count: u32 = 0;

    let directions: Vec<(i16, i16)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let rows = array.len();
    let cols = array[0].len();
    for row in 0..rows {
        for col in 0..cols {
            let current_height = array[row][col];
            if current_height != 0 {
                continue;
            }
            let mut hilltops = HashSet::new();
            let starting_point = (row as i16, col as i16);
            iterate_directions(
                &array,
                starting_point,
                current_height,
                &mut |next_point| {
                    hilltops.insert(next_point);
                },
                &directions,
            );
            count += hilltops.len() as u32;
        }
    }

    Some(count as u32)
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut array = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as u8);
        }
        array.push(row);
    }
    array
}

fn dfs(
    array: &Vec<Vec<u8>>,
    next_point: (i16, i16),
    previous_height: u8,
    on_hilltop_reached: &mut dyn FnMut((i16, i16)) -> (),
    directions: &Vec<(i16, i16)>,
) {
    let (x, y) = next_point;
    let rows = array.len() as i16;
    let cols = array[0].len() as i16;

    if x >= rows || y >= cols {
        return;
    }

    let current_height = array[x as usize][y as usize];
    if current_height == 9 && previous_height == 8 {
        on_hilltop_reached(next_point);
        return;
    }
    let is_new_trailhead = current_height == 0;
    let is_non_strict_increment =
        current_height != 0 && current_height.saturating_sub(previous_height) != 1;
    if is_new_trailhead || is_non_strict_increment {
        return;
    }
    println!("Current: {current_height} at {x};{y}");

    iterate_directions(
        array,
        next_point,
        current_height,
        on_hilltop_reached,
        directions,
    );
}

fn iterate_directions(
    array: &Vec<Vec<u8>>,
    current_point: (i16, i16),
    current_height: u8,
    on_hilltop_reached: &mut dyn FnMut((i16, i16)) -> (),
    directions: &Vec<(i16, i16)>,
) {
    let (x, y) = current_point;
    for (dx, dy) in directions.iter() {
        let new_x = x + dx;
        let new_y = y + dy;

        if new_x < 0 || new_y < 0 {
            continue;
        }
        println!("moving to {new_x};{new_y}");

        dfs(
            array,
            (new_x as i16, new_y as i16),
            current_height,
            on_hilltop_reached,
            directions,
        );
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let array = parse_input(input);

    let mut count: u32 = 0;

    let directions: Vec<(i16, i16)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let rows = array.len();
    let cols = array[0].len();
    for row in 0..rows {
        for col in 0..cols {
            let current_height = array[row][col];
            if current_height != 0 {
                continue;
            }
            let starting_point = (row as i16, col as i16);
            iterate_directions(
                &array,
                starting_point,
                current_height,
                &mut |_| {
                    count += 1;
                },
                &directions,
            );
        }
    }

    Some(count as u32)
}
