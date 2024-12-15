use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (antenna_positions_by_frequency, mut array) = parse_input(input);
    let mut antinode_positions: HashSet<(char, i32, i32)> = HashSet::new();

    let rows = array.len() as i32;
    let cols = array[0].len() as i32;

    for (frequency, antenna_positions) in antenna_positions_by_frequency {
        let combinations = antenna_positions
            .iter()
            .combinations(2)
            .map(|c| (c[0], c[1]));
        for ((x1, y1), (x2, y2)) in combinations {
            check_antinode_positions(
                frequency,
                *x1 as i32,
                *x2 as i32,
                *y1 as i32,
                *y2 as i32,
                rows,
                cols,
                &mut array,
                &mut antinode_positions,
                false,
            );
        }
    }

    Some(antinode_positions.len() as u32)
}

fn check_antinode_positions(
    frequency: char,
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    rows: i32,
    cols: i32,
    array: &mut Vec<Vec<char>>,
    antinode_positions: &mut HashSet<(char, i32, i32)>,
    recursively: bool,
) {
    let dx1 = x1 as i32 - x2 as i32;
    let dy1 = y1 as i32 - y2 as i32;
    let new_x_1 = x1 as i32 + dx1;
    let new_y_1 = y1 as i32 + dy1;
    let dx2 = x2 as i32 - x1 as i32;
    let dy2 = y2 as i32 - y1 as i32;
    let new_x_2 = x2 as i32 + dx2;
    let new_y_2 = y2 as i32 + dy2;
    println!("combination ({x1}, {y1}) with ({x2}, {y2}) would lead to ({new_x_1}, {new_y_1}) and ({new_x_2}, {new_y_2})");

    let antinode_1_in_bounds = new_x_1 >= 0 && new_x_1 < rows && new_y_1 >= 0 && new_y_1 < cols;
    let antinode_1_pos = (frequency, new_x_1, new_y_1);
    let mut antinode_1_needs_recursion = false;
    if antinode_1_in_bounds && !antinode_positions.contains(&antinode_1_pos) {
        println!("antinode 1 ({new_x_1}, {new_y_1}): {antinode_1_in_bounds}");
        array[new_x_1 as usize][new_y_1 as usize] = '#';
        antinode_1_needs_recursion = !antinode_positions.contains(&antinode_1_pos);
        antinode_positions.insert(antinode_1_pos);
    }
    let antinode_2_in_bounds = new_x_2 >= 0 && new_x_2 < rows && new_y_2 >= 0 && new_y_2 < cols;
    let antinode_2_pos = (frequency, new_x_2, new_y_2);
    let mut antinode_2_needs_recursion = false;
    if antinode_2_in_bounds && !antinode_positions.contains(&antinode_2_pos) {
        println!("antinode 2 ({new_x_2}, {new_y_2}): {antinode_2_in_bounds}");
        array[new_x_2 as usize][new_y_2 as usize] = '#';
        antinode_2_needs_recursion = !antinode_positions.contains(&antinode_2_pos);
        antinode_positions.insert(antinode_2_pos);
    }

    if antinode_1_in_bounds && antinode_1_needs_recursion && recursively {
        check_antinode_positions(
            frequency,
            new_x_1,
            x1,
            new_y_1,
            y1,
            rows,
            cols,
            array,
            antinode_positions,
            true,
        );
    }
    if antinode_2_in_bounds && antinode_2_needs_recursion && recursively {
        check_antinode_positions(
            frequency,
            new_x_2,
            x2,
            new_y_2,
            y2,
            rows,
            cols,
            array,
            antinode_positions,
            true,
        );
    }
}

fn parse_input(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, Vec<Vec<char>>) {
    let mut array: Vec<Vec<char>> = Vec::new();
    let mut hash_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (row_index, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (column_index, char) in line.chars().enumerate() {
            row.push(char);
            if char != '.' {
                let pos = (row_index, column_index);
                hash_map
                    .entry(char)
                    .and_modify(|v| v.push(pos))
                    .or_insert(vec![pos]);
            }
        }
        array.push(row);
    }
    (hash_map, array)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antenna_positions_by_frequency, mut array) = parse_input(input);
    let mut antinode_positions: HashSet<(char, i32, i32)> = HashSet::new();
    let initial_antenna_positions = antenna_positions_by_frequency
        .iter()
        .flat_map(|(_, v)| v.iter().map(|(x, y)| (*x as i32, *y as i32)))
        .collect_vec();

    let rows = array.len() as i32;
    let cols = array[0].len() as i32;

    for (frequency, antenna_positions) in antenna_positions_by_frequency {
        let combinations = antenna_positions
            .iter()
            .combinations(2)
            .map(|c| (c[0], c[1]));
        for ((x1, y1), (x2, y2)) in combinations {
            check_antinode_positions(
                frequency,
                *x1 as i32,
                *x2 as i32,
                *y1 as i32,
                *y2 as i32,
                rows,
                cols,
                &mut array,
                &mut antinode_positions,
                true,
            );
        }
    }

    print_array(&array);

    let mut uniq: HashSet<(&i32, &i32)> =
        HashSet::from_iter(antinode_positions.iter().map(|(_, x, y)| (x, y)));
    for initial_pos in &initial_antenna_positions {
        uniq.insert((&initial_pos.0, &initial_pos.1));
    }

    Some(uniq.len() as u32)
}

fn print_array(array: &Vec<Vec<char>>) {
    for row in array {
        println!("{:?}", row.iter().join(""));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
