use itertools::Itertools;
use std::iter::repeat_n;

advent_of_code::solution!(7);

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = parse_input(input);
    let mut total = 0;
    for (solution, operands) in parsed {
        println!("{solution} = {:?}", operands);

        let op_len = operands.len() - 1;
        let available_ops = vec![Op::Add, Op::Mul];
        validate(available_ops, op_len, operands, solution, &mut total);
    }
    Some(total)
}

fn validate(
    available_ops: Vec<Op>,
    op_len: usize,
    operands: Vec<u64>,
    solution: u64,
    total: &mut u64,
) {
    let combinations = repeat_n(available_ops.iter(), op_len).multi_cartesian_product();
    for ops in combinations {
        let mut ops_iter = ops.iter();
        println!("{:?}", ops);
        let result = operands
            .clone()
            .into_iter()
            .reduce(|prev, next| match ops_iter.next() {
                Some(Op::Add) => prev + next,
                Some(Op::Mul) => prev * next,
                Some(Op::Concat) => format!("{prev}{next}").parse().unwrap(),
                _ => panic!("End reached without solving"),
            });
        if result.unwrap() == solution {
            println!("Solution found!");
            *total += solution;
            break;
        }
    }
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut equations: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in input.lines() {
        let split = line.split(':').collect_vec();
        println!("{:?}", split[0]);
        let solution = split[0].parse::<u64>().unwrap();
        let operands: Vec<u64> = split[1]
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        equations.push((solution, operands));
    }
    equations
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = parse_input(input);
    let mut total = 0;
    for (solution, operands) in parsed {
        println!("{solution} = {:?}", operands);

        let op_len = operands.len() - 1;
        let available_ops = vec![Op::Add, Op::Mul, Op::Concat];
        validate(available_ops, op_len, operands, solution, &mut total);
    }
    Some(total)
}
