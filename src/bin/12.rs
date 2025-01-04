use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Debug)]
struct Region {
    area: u32,
    perimeter: u32,
    corners: u32,
    inner_corners: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let array = parse_input(input);

    let directions: Vec<(i16, i16)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let regions = find_regions(array, directions);

    let sum = regions
        .iter()
        .map(
            |Region {
                 area, perimeter, ..
             }| area * perimeter,
        )
        .sum::<u32>();
    Some(sum)
}

fn find_regions(array: Vec<Vec<char>>, directions: Vec<(i16, i16)>) -> Vec<Region> {
    let mut visited_points: HashSet<(i16, i16)> = HashSet::new();
    let mut regions: Vec<Region> = Vec::new();
    let rows = array.len();
    let cols = array[0].len();
    for row in 0..rows {
        for col in 0..cols {
            let start = (row as i16, col as i16);
            let current_label = array[row][col];

            if visited_points.contains(&start) {
                continue;
            }

            let mut current_region = Region {
                area: 0,
                perimeter: 0,
                corners: 0,
                inner_corners: 0,
            };

            for (dx, dy) in directions.iter() {
                let new_x = row as i16 + dx;
                let new_y = col as i16 + dy;

                if new_x < 0 || new_y < 0 {
                    continue;
                }

                dfs(
                    &array,
                    current_label,
                    start,
                    &mut visited_points,
                    &mut current_region,
                    &directions,
                );
            }

            println!("Discovered region {:?}", current_region);

            regions.push(current_region);
        }
    }
    regions
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut array: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        array.push(row);
    }
    array
}

fn dfs(
    array: &Vec<Vec<char>>,
    current_label: char,
    start: (i16, i16),
    visited_points: &mut HashSet<(i16, i16)>,
    current_region: &mut Region,
    directions: &Vec<(i16, i16)>,
) -> Option<char> {
    let (x, y) = start;
    let rows = array.len() as i16;
    let cols = array[0].len() as i16;

    if x >= rows || y >= cols {
        return None;
    }

    let next_label = array[x as usize][y as usize];

    if visited_points.contains(&start) || next_label != current_label {
        return Some(next_label);
    }
    // sprintln!("Current: {next_label} at {x};{y}");

    visited_points.insert(start);
    current_region.area += 1;

    let mut same_region_neighbors = HashSet::new();
    for (dx, dy) in directions.iter() {
        let new_x = x as i16 + dx;
        let new_y = y as i16 + dy;

        if new_x < 0 || new_y < 0 {
            continue;
        }

        let neighbor_label = dfs(
            &array,
            current_label,
            (new_x, new_y),
            visited_points,
            current_region,
            &directions,
        );
        if neighbor_label.is_some_and(|l| l == current_label) {
            same_region_neighbors.insert((new_x, new_y));
        }
    }

    /*println!(
        "no. same region neighbors at {x};{y}: {:?}",
        same_region_neighbors.len()
    );*/
    match same_region_neighbors.len() {
        0 => {
            current_region.perimeter += 4;
            current_region.corners += 4;
        }
        1 => {
            current_region.perimeter += 3;
            current_region.corners += 2;
        }
        2 => {
            current_region.perimeter += 2;
            let ((n1_x, n1_y), (n2_x, n2_y)) = same_region_neighbors.iter().next_tuple().unwrap();
            // the neighbors are NOT on opposite sides
            if n1_x != n2_x && n1_y != n2_y {
                current_region.corners += 1;
            }
        }
        3 => {
            current_region.perimeter += 1;
        }
        _ => {
            // do not consider for the default perimeter as this is an inner plot
        }
    }

    // count inner corners
    same_region_neighbors
        .iter()
        .tuple_combinations()
        .for_each(|((n1_x, n1_y), (n2_x, n2_y))| {
            if n1_x == n2_x || n1_y == n2_y {
                return;
            }

            let new_x = n1_x + n2_x - x;
            let new_y = n1_y + n2_y - y;

            if new_x < 0 || new_y < 0 {
                return;
            }

            let diagonal = (new_x, new_y);
            let neighbor_label = dfs(
                &array,
                current_label,
                diagonal,
                visited_points,
                current_region,
                &directions,
            );
            //neighbor_label.map(|l| println!("Neighbor pair diagonal label {l} at {new_x};{new_y}"));
            if neighbor_label.is_some_and(|l| l != current_label) {
                current_region.inner_corners += 1;
            }
        });

    Some(next_label)
}

pub fn part_two(input: &str) -> Option<u32> {
    let array = parse_input(input);

    let directions: Vec<(i16, i16)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let regions = find_regions(array, directions);

    let sum = regions
        .iter()
        .map(
            |Region {
                 area,
                 corners,
                 inner_corners,
                 ..
             }| area * (corners + inner_corners),
        )
        .sum::<u32>();
    Some(sum)
}
