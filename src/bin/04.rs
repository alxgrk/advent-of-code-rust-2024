advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let array = parse_input(input);

    let mut current_chars: Vec<char> = Vec::new();
    let mut count: u32 = 0;

    let directions: Vec<(i16, i16)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    let rows = array.len();
    let cols = array[0].len();
    for row in 0..rows {
        for col in 0..cols {
            let start = (row as i16, col as i16);
            dfs(&array, start, &mut current_chars, &mut count, &directions);
        }
    }

    Some(count as u32)
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
    start: (i16, i16),
    current_chars: &Vec<char>,
    count: &mut u32,
    directions: &Vec<(i16, i16)>,
) {
    let mut current_chars = current_chars.clone();

    let (x, y) = start;
    let rows = array.len() as i16;
    let cols = array[0].len() as i16;

    if x >= rows || y >= cols {
        return;
    }

    let current_char = array[x as usize][y as usize];
    println!("Current: {current_char} at {x};{y}");

    match current_chars.as_slice() {
        [x, m, a] => {
            if x == &'X' && m == &'M' && a == &'A' && current_char == 'S' {
                println!("Found S!");
                *count += 1;
                return;
            } else {
                println!("No match!");
                return;
            }
        }
        [x, m] => {
            if x == &'X' && m == &'M' && current_char == 'A' {
                println!("Found A!");
                current_chars.push(current_char);
            } else {
                println!("No match!");
                return;
            }
        }
        [x] => {
            if x == &'X' && current_char == 'M' {
                println!("Found M!");
                current_chars.push(current_char);
            } else {
                println!("No match!");
                return;
            }
        }
        [] => {
            if current_char == 'X' {
                println!("Found X!");
                current_chars.push(current_char);
            } else {
                println!("No match!");
                return;
            }
        }
        _ => println!("No match found"),
    }
    println!("{:?}", current_chars);

    for (dx, dy) in directions.iter() {
        let new_x = x as i16 + dx;
        let new_y = y as i16 + dy;

        if new_x < 0 || new_y < 0 {
            continue;
        }
        println!("moving to {new_x};{new_y}");

        dfs(
            array,
            (new_x as i16, new_y as i16),
            &current_chars,
            count,
            &vec![(*dx, *dy)],
        );
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let array = parse_input(input);

    let mut count: u32 = 0;

    let rows = array.len();
    let cols = array[0].len();
    // chars at the edge can be ignored
    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            let current_char = array[row][col];

            if current_char == 'A' {
                let nw = array[row - 1][col - 1];
                let ne = array[row + 1][col - 1];
                let sw = array[row - 1][col + 1];
                let se = array[row + 1][col + 1];

                if (nw == 'M' && ne == 'M' && se == 'S' && sw == 'S')
                    || (sw == 'M' && se == 'M' && ne == 'S' && nw == 'S')
                    || (sw == 'M' && nw == 'M' && ne == 'S' && se == 'S')
                    || (se == 'M' && ne == 'M' && sw == 'S' && nw == 'S')
                {
                    count += 1;
                }
            }
        }
    }

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
