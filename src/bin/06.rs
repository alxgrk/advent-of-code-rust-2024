advent_of_code::solution!(6);

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Obstacle,
    Free,
    Visited,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (guard_position, mut array) = parse_input(input);
    println!("start: {:?}", guard_position);

    let cnt = run_part_one(guard_position, &mut array);
    Some(cnt)
}

fn run_part_one(mut guard_position: (usize, usize), array: &mut Vec<Vec<Field>>) -> u32 {
    let mut next_step = (-1, 0);
    let mut cnt = 1;
    while let Some(new_position) = guard_has_gone(guard_position, &array, next_step) {
        println!("new position: {:?}", new_position);
        let field = &array[new_position.0][new_position.1];
        match field {
            Field::Free => {
                array[new_position.0][new_position.1] = Field::Visited;
                guard_position = new_position;
                cnt += 1;
                println!("free! {cnt}");
            }
            Field::Visited => {
                guard_position = new_position;
                println!("visited! {cnt}");
            }
            Field::Obstacle => {
                println!("obstacle! {cnt}");
                next_step = match next_step {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => panic!("Unexpected next step"),
                };
            }
        }
    }
    cnt
}

fn guard_has_gone(
    guard_position: (usize, usize),
    array: &Vec<Vec<Field>>,
    next_step: (i32, i32),
) -> Option<(usize, usize)> {
    let rows = array.len() as i32;
    let cols = array[0].len() as i32;
    let (row, col) = guard_position;
    let (next_row, next_col) = next_step;
    let new_row = (row as i32) + next_row;
    let new_col = (col as i32) + next_col;
    if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
        None
    } else {
        Some((new_row as usize, new_col as usize))
    }
}

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<Field>>) {
    let mut guard_position: (usize, usize) = (0, 0);
    let mut array: Vec<Vec<Field>> = Vec::new();
    for (row_index, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (column_index, char) in line.chars().enumerate() {
            match char {
                '.' => row.push(Field::Free),
                '#' => row.push(Field::Obstacle),
                '^' => {
                    row.push(Field::Visited);
                    guard_position = (row_index, column_index)
                }
                _ => panic!("Unexpected char"),
            }
        }
        array.push(row);
    }
    (guard_position, array)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (guard_position, array) = parse_input(input);
    println!("start: {:?}", guard_position);
    let initial_guard_position = guard_position.clone();

    let mut part_one_array = array.clone();
    let guard_position = guard_position.clone();
    run_part_one(guard_position, &mut part_one_array);

    let mut cnt = 0;
    let rows = array.len();
    let cols = array[0].len();
    for row in 0..rows {
        for col in 0..cols {
            let possible_obstacle = (row, col);
            if initial_guard_position == possible_obstacle
                || part_one_array[row][col] != Field::Visited
            {
                println!("skipping: {row},{col}");
                continue;
            }

            let mut array = array.clone();
            let mut guard_position = guard_position.clone();
            array[row][col] = Field::Obstacle;
            println!("possible obstacle added: {:?}", possible_obstacle);
            let mut known_position_direction_pairs = vec![];

            let mut next_step = (-1, 0);
            while let Some(new_position) = guard_has_gone(guard_position, &array, next_step) {
                //println!("new position: {:?}", new_position);
                let position_direction_pair = (new_position, next_step);
                if known_position_direction_pairs.contains(&position_direction_pair) {
                    println!("loop detected at: {row},{col}");
                    cnt += 1;
                    break;
                }

                let field = &array[new_position.0][new_position.1];
                match field {
                    Field::Free => {
                        array[new_position.0][new_position.1] = Field::Visited;
                        guard_position = new_position;
                        known_position_direction_pairs.push(position_direction_pair);
                        //println!("free!");
                    }
                    Field::Visited => {
                        guard_position = new_position;
                        known_position_direction_pairs.push(position_direction_pair);
                        //println!("visited!");
                    }
                    Field::Obstacle => {
                        //println!("obstacle!");
                        next_step = match next_step {
                            (-1, 0) => (0, 1),
                            (0, 1) => (1, 0),
                            (1, 0) => (0, -1),
                            (0, -1) => (-1, 0),
                            _ => panic!("Unexpected next step"),
                        };
                    }
                }
            }
        }
    }
    Some(cnt)
}
