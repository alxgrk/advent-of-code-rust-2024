use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    input
        .lines()
        .flat_map(|line| {
            re.captures_iter(line)
                .map(|c| {
                    let (_, [left, right]) = c.extract();
                    left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap()
                })
                .reduce(|prev, next| prev + next)
        })
        .reduce(|prev, next| prev + next)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut is_active = true;
    let mut result = 0;
    input.lines().for_each(|line| {
        let mut instructions: Vec<Instruction> = Vec::new();

        re.captures_iter(line).for_each(|c| {
            let position = c.get(0).unwrap().start();
            let (_, [left, right]) = c.extract();
            let value = Some(left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap());
            instructions.push(Instruction {
                position,
                op: Op::MUL,
                value,
            });
        });
        do_regex.captures_iter(line).for_each(|c| {
            let position = c.get(0).unwrap().start();
            instructions.push(Instruction {
                position,
                op: Op::DO,
                value: None,
            });
        });
        dont_regex.captures_iter(line).for_each(|c| {
            let position = c.get(0).unwrap().start();
            instructions.push(Instruction {
                position,
                op: Op::DONT,
                value: None,
            });
        });

        instructions.sort_by_key(|i| i.position);
        for i in instructions {
            match i.op {
                Op::MUL => {
                    if is_active {
                        result += i.value.unwrap();
                    }
                }
                Op::DO => is_active = true,
                Op::DONT => is_active = false,
            }
        }
    });
    Some(result)
}

#[derive(Debug)]
enum Op {
    MUL,
    DO,
    DONT,
}

#[derive(Debug)]
struct Instruction {
    position: usize,
    op: Op,
    value: Option<u32>,
}
