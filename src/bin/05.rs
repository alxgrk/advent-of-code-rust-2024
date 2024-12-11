use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    println!("rules: {:?}", rules);
    println!("updates: {:?}", updates);

    let result: i32 = updates
        .iter()
        .filter(|update| validate(&update, get_ordered_numbers_from_rules(&update, &rules)))
        .map(|update| update[update.len() / 2])
        .sum();
    Some(result as u32)
}

fn get_ordered_numbers_from_rules(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut ordered_numbers: Vec<i32> = update.iter().cloned().collect();
    let mut is_before_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (i1, i2) in rules {
        if !ordered_numbers.contains(&i1) && !ordered_numbers.contains(&i2) {
            continue;
        }
        is_before_map
            .entry(*i1)
            .and_modify(|v| {
                v.insert(*i2);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(*i2);
                set
            });
    }
    ordered_numbers.sort_by(|&i1, &i2| {
        if is_before(&is_before_map, &i1, &i2) {
            Ordering::Less
        } else if is_before(&is_before_map, &i2, &i1) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    println!("ordered: {:?}", ordered_numbers);
    ordered_numbers
}

fn is_before(is_before_map: &HashMap<i32, HashSet<i32>>, i1: &i32, i2: &i32) -> bool {
    is_before_map
        .get(i1)
        .map(|v| v.contains(i2))
        .unwrap_or(false)
}

fn index_of(vec: &Vec<i32>, item: &i32) -> usize {
    vec.iter().position(|&r| r == *item).unwrap()
}

fn validate(update: &Vec<i32>, ordered_numbers: Vec<i32>) -> bool {
    let all = update.windows(2).all(|w| {
        let is_ok = index_of(&ordered_numbers, &w[0]) < index_of(&ordered_numbers, &w[1]);
        //println!("is ok: {is_ok}");
        is_ok
    });
    if all {
        println!("update: {:?}", update);
    }
    all
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let is_rule = line.contains('|');
        let is_update = line.contains(',');

        if is_rule {
            let rule: (i32, i32) = line
                .split('|')
                .map(|i| i.parse().unwrap())
                .collect_tuple()
                .unwrap();
            rules.push(rule);
        } else if is_update {
            let update = line.split(',').map(|i| i.parse().unwrap()).collect();
            updates.push(update);
        }
    }
    (rules, updates)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    println!("rules: {:?}", rules);
    println!("updates: {:?}", updates);

    let result: i32 = updates
        .iter()
        .map(|update| (update, get_ordered_numbers_from_rules(&update, &rules)))
        .filter(|(update, ordered_numbers)| *update != ordered_numbers)
        .map(|(_, ordered_numbers)| ordered_numbers)
        .map(|update| update[update.len() / 2])
        .sum();
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
