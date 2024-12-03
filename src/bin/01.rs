use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (lefts, rights) = parse_input(input);
    let mut total_distance = 0;
    for i in 0..lefts.len() {
        total_distance += (lefts[i] - rights[i]).abs();
    }
    Some(total_distance.try_into().unwrap())
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: Vec<i32> = Vec::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        match split.as_slice() {
            [left, right] => {
                lefts.push(left.parse().unwrap());
                rights.push(right.parse().unwrap());
            }
            _ => println!("Das Vec hat nicht genau 2 Elemente."),
        }
    }
    lefts.sort();
    rights.sort();
    (lefts, rights)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (lefts, rights) = parse_input(input);
    let occurences = rights
        .iter()
        .into_grouping_map_by(|&r| r)
        .fold(0, |acc, _k, _v| acc + 1);

    let mut total_distance = 0;
    for l in lefts {
        let times = occurences.get(&l).unwrap_or(&0);
        total_distance += l * times;
    }
    Some(total_distance.try_into().unwrap())
}
