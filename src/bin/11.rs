use std::{collections::HashMap, hash::Hash, thread::current, time::Instant};

use itertools::Itertools;

advent_of_code::solution!(11);

fn first_rule(number: &u64) -> Option<Vec<u64>> {
    if number == &0 {
        //println!("1. rule matched");
        Some(vec![1])
    } else {
        None
    }
}

fn second_rule(number: &u64) -> Option<Vec<u64>> {
    let mut as_string = number.to_string();
    if as_string.len() % 2 == 0 {
        //println!("2. rule matched");
        let second_half = as_string.split_off(as_string.len() / 2);
        Some(vec![
            as_string.parse().unwrap(),
            second_half.parse().unwrap(),
        ])
    } else {
        None
    }
}

fn third_rule(number: &u64) -> Option<Vec<u64>> {
    //println!("3. rule matched");
    Some(vec![number * 2024])
}

pub fn part_one(input: &str) -> Option<u64> {
    solve_naive(input, 25)
}

fn solve_naive(input: &str, rounds: u8) -> Option<u64> {
    let rules: Vec<&dyn Fn(&u64) -> Option<Vec<u64>>> =
        vec![&first_rule, &second_rule, &third_rule];

    let initial_numbers = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|d| d.parse::<u64>().unwrap())
        .collect_vec();

    let mut memory: HashMap<u64, Vec<u64>> = HashMap::new();

    let mut current_numbers = initial_numbers.clone();
    let mut next_numbers = Vec::new();
    for round in 0..rounds {
        println!("{round}. round with {:0} numbers", current_numbers.len());
        let start = Instant::now();
        for number in &current_numbers {
            //println!("current number: {number}");
            if let Some(numbers) = memory.get(number) {
                numbers.iter().for_each(|n| next_numbers.push(*n));
                continue;
            }
            for rule in &rules {
                if let Some(numbers) = rule(number) {
                    numbers.iter().for_each(|n| next_numbers.push(*n));
                    memory.insert(*number, numbers);
                    break;
                }
            }
        }
        current_numbers = next_numbers;
        next_numbers = Vec::with_capacity(current_numbers.capacity() * 2);
        println!("Took {:0}ms", start.elapsed().as_millis());
    }

    Some(current_numbers.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_advanced(input, 75)
}

fn solve_advanced(input: &str, rounds: u8) -> Option<u64> {
    let rules: Vec<&dyn Fn(&u64) -> Option<Vec<u64>>> =
        vec![&first_rule, &second_rule, &third_rule];

    let initial_numbers = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|d| d.parse::<u64>().unwrap())
        .collect_vec();

    let mut memory: HashMap<u64, usize> = HashMap::new();
    initial_numbers.iter().for_each(|n| {
        memory.insert(*n, 1);
    });
    let mut next_round_memory: HashMap<u64, usize> = HashMap::new();
    for round in 0..rounds {
        println!(
            "{round}. round with {:0} numbers",
            memory.values().sum::<usize>()
        );
        let start = Instant::now();
        for (number, count) in &memory {
            // println!("current number: {number}");
            for rule in &rules {
                if let Some(numbers) = rule(number) {
                    numbers.iter().for_each(|n| {
                        next_round_memory
                            .entry(*n)
                            .and_modify(|e| *e += count)
                            .or_insert(*count);
                    });
                    break;
                }
            }
        }
        memory = next_round_memory;
        next_round_memory = HashMap::new();
        println!("Took {:0}ms", start.elapsed().as_millis());
    }

    Some(memory.values().sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
