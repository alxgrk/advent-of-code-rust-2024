use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(14);

const DEFAULT_WIDTH: i16 = 101;
const DEFAULT_HEIGHT: i16 = 103;

const TEST_WIDTH: i16 = 11;
const TEST_HEIGHT: i16 = 7;

pub fn part_one(input: &str) -> Option<u32> {
    let max_width = DEFAULT_WIDTH;
    let max_height = DEFAULT_HEIGHT;
    // let max_width = TEST_WIDTH;
    // let max_height = TEST_HEIGHT;

    let mut robots = parse(input);

    // println!("robots: {:?}", robots.iter().map(|(p, _v)| p).collect_vec());

    for _ in 0..100 {
        robots = move_robots(max_width, max_height, &robots);
        // println!("robots: {:?}", robots.iter().map(|(p, _v)| p).collect_vec());
    }

    let safety_factor = calc_safety_factor(max_width, max_height, &robots);

    Some(safety_factor)
}

fn calc_safety_factor(
    max_width: i16,
    max_height: i16,
    robots: &Vec<((i16, i16), (i16, i16))>,
) -> u32 {
    let m_width = max_width / 2;
    let m_height = max_height / 2;
    let safety_factor = robots
        .iter()
        .filter(|((x, y), _)| *x != m_width && *y != m_height)
        .fold([0; 4], |[a, b, c, d], ((x, y), _)| {
            [
                if *x < m_width && *y < m_height {
                    a + 1
                } else {
                    a
                },
                if *x > m_width && *y < m_height {
                    b + 1
                } else {
                    b
                },
                if *x < m_width && *y > m_height {
                    c + 1
                } else {
                    c
                },
                if *x > m_width && *y > m_height {
                    d + 1
                } else {
                    d
                },
            ]
        })
        .iter()
        .fold(1, |acc, factor| acc * factor);
    safety_factor
}

fn move_robots(
    max_width: i16,
    max_height: i16,
    robots: &Vec<((i16, i16), (i16, i16))>,
) -> Vec<((i16, i16), (i16, i16))> {
    robots
        .iter()
        .map(|(position, velocity)| {
            let (x, y) = position;
            let (dx, dy) = velocity;

            let mut new_x = x + dx;
            if new_x >= max_width {
                new_x -= max_width; // e.g. new_x = 11 -> 11 - 10 = 1
            } else if new_x < 0 {
                new_x += max_width; // e.g. new_x = -1 -> 10 + -1 = 9
            };
            let mut new_y = y + dy;
            if new_y >= max_height {
                new_y -= max_height; // e.g. new_y = 11 -> 11 - 10 = 1
            } else if new_y < 0 {
                new_y += max_height; // e.g. new_y = -1 -> 10 + -1 = 9
            };
            ((new_x, new_y), *velocity)
        })
        .collect()
}

fn parse(input: &str) -> Vec<((i16, i16), (i16, i16))> {
    let robots = input
        .lines()
        .map(|line| {
            let parsed = line
                .split_whitespace()
                .map(|p| {
                    let split = p.split("=").last().unwrap().split(",").collect_vec();
                    let x = split[0].parse::<i16>().unwrap();
                    let y = split[1].parse::<i16>().unwrap();
                    (x, y)
                })
                .collect_vec();
            (parsed[0], parsed[1])
        })
        .collect_vec();
    robots
}

pub fn part_two(input: &str) -> Option<u32> {
    // the christmas tree exists at the moment, when each robot is alone at its tile

    let max_width = DEFAULT_WIDTH;
    let max_height = DEFAULT_HEIGHT;
    // let max_width = TEST_WIDTH;
    // let max_height = TEST_HEIGHT;

    let mut robots = parse(input);

    // println!("robots: {:?}", robots.iter().map(|(p, _v)| p).collect_vec());

    let mut safety_factors = vec![];
    loop {
        robots = move_robots(max_width, max_height, &robots);

        let safety_factor = calc_safety_factor(max_width, max_height, &robots);
        safety_factors.push((safety_factors.len() + 1, safety_factor, robots.clone()));

        // let positions_as_set: HashSet<(i16, i16)> = robots.iter().map(|(p, _v)| *p).collect();
        if safety_factors.len() >= 10000 {
            break;
        }
    }

    let solution = safety_factors
        .iter()
        .sorted_by_key(|(_iteration, safety_factor, ..)| *safety_factor)
        .take(1)
        .map(|(iteration, _safety_factor, robots)| {
            print_plan(max_width, max_height, robots);
            *iteration
        });

    let collect_vec = solution.collect_vec();
    Some(*collect_vec.first().unwrap() as u32)
}

fn print_plan(max_width: i16, max_height: i16, robots: &Vec<((i16, i16), (i16, i16))>) {
    for i in 0..max_height {
        for j in 0..max_width {
            let count = robots
                .iter()
                .filter(|((x, y), _)| j == *x && i == *y)
                .count();
            if count > 0 {
                print!("{count}");
            } else {
                print!(".")
            }
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
