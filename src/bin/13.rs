advent_of_code::solution!(13);

use itertools::Itertools;

const BUTTON_A_COST: u64 = 3;
const BUTTON_B_COST: u64 = 1;

pub fn part_one(input: &str) -> Option<u64> {
    let mut a = input.lines().filter(|l| !l.trim().is_empty());
    let mut sum = 0u64;

    while let Some(button_a_line) = a.next() {
        let button_a_line = parse_line(button_a_line, '+');
        let button_b_line = parse_line(a.next().unwrap(), '+');
        let prize_line = parse_line(a.next().unwrap(), '=');

        calc(&mut sum, button_a_line, button_b_line, prize_line);
    }
    Some(sum)
}

fn calc(
    sum: &mut u64,
    button_a_line: (u64, u64),
    button_b_line: (u64, u64),
    prize_line: (u64, u64),
) {
    let det = determinant(button_a_line, button_b_line);
    let det_a = determinant(prize_line, button_b_line);
    let det_b = determinant(prize_line, button_a_line);

    let a_rem = det_a % &det;
    let b_rem = det_b % &det;

    if a_rem == 0 && b_rem == 0 {
        let n_press_a = det_a / &det;
        let n_press_b = det_b / &det;
        let cost_button_a = BUTTON_A_COST * n_press_a;
        let cost_button_b = BUTTON_B_COST * n_press_b;
        println!("Result: {n_press_a}xA, {n_press_b}xB -> {cost_button_a} + {cost_button_b}");
        *sum += cost_button_a + cost_button_b;
    }
}

fn parse_line(line: &str, sep: char) -> (u64, u64) {
    let x_and_y = line
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(", ")
        .collect_vec();
    let x = x_and_y[0].split(sep).last().unwrap().parse().unwrap();
    let y = x_and_y[1].split(sep).last().unwrap().parse().unwrap();
    (x, y)
}

// Function to calculate the determinant of a 2x2 matrix
fn determinant(row1: (u64, u64), row2: (u64, u64)) -> u64 {
    let (a, b) = row1;
    let (c, d) = row2;
    (a as i64 * d as i64 - b as i64 * c as i64).abs() as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut a = input.lines().filter(|l| !l.trim().is_empty());
    let mut sum = 0u64;

    while let Some(button_a_line) = a.next() {
        let button_a_line = parse_line(button_a_line, '+');
        let button_b_line = parse_line(a.next().unwrap(), '+');
        let prize_line = parse_line(a.next().unwrap(), '=');

        let corrected_prize_line = (
            prize_line.0 + 10_000_000_000_000,
            prize_line.1 + 10_000_000_000_000,
        );

        calc(&mut sum, button_a_line, button_b_line, corrected_prize_line);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
