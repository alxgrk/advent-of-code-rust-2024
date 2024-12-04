use itertools::Itertools;

advent_of_code::solution!(2);

#[derive(Debug)]
enum Direction {
    ASC,
    DESC,
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let result: Vec<&Vec<i32>> = reports.iter().filter(|&report| is_valid(report)).collect();
    Some(result.len().try_into().unwrap())
}

fn is_valid(report: &Vec<i32>) -> bool {
    if report[0] == report[1] {
        return false;
    }
    let direction: Direction = if report[0] > report[1] {
        Direction::DESC
    } else {
        Direction::ASC
    };
    report
        .iter()
        .reduce(|value, next| {
            if value == &-1 {
                return value;
            }
            let diff = match direction {
                Direction::ASC => next - value,
                Direction::DESC => value - next,
            };
            if diff <= 3 && diff > 0 {
                next
            } else {
                &-1
            }
        })
        .unwrap()
        != &-1
}

fn dampen(report: &Vec<i32>) -> Vec<Vec<i32>> {
    (0..report.len())
        .map(|i| {
            let mut r = report.clone();
            r.remove(i);
            r
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let initially_valid: Vec<&Vec<i32>> =
        reports.iter().filter(|&report| is_valid(report)).collect();

    let dampened_valid: Vec<&Vec<i32>> = reports
        .iter()
        .filter(|r| !is_valid(r))
        .filter(|r| dampen(r).iter().any(|r| is_valid(r)))
        .collect();
    Some(
        (initially_valid.len() + dampened_valid.len())
            .try_into()
            .unwrap(),
    )
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect()
        })
        .sorted()
        .collect()
}
