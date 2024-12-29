use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    File(u64),
    Free,
}

pub fn part_one(input: &str) -> Option<u64> {
    let layout = parse_layout(input);
    //println!("{layout}");

    let mut poppable = layout.clone();
    let mut compacted = Vec::new();
    for (rev_index, block) in layout.iter().enumerate().rev() {
        //println!("char: {char}");
        if Block::Free == *block {
            continue;
        }
        if 0 == poppable.len() || compacted.len() > rev_index {
            break;
        }
        let mut next = poppable.drain(0..1).last();
        while let Some(next_block) = next {
            if 0 == poppable.len() || compacted.len() > rev_index {
                break;
            }
            //println!("first: {next_char}");
            if Block::Free != next_block {
                compacted.push(next_block);
                next = poppable.drain(0..1).last();
            } else {
                compacted.push(*block);
                next = None;
            }
            //println!("compacted: {compacted}");
        }
    }
    println!("final: {}", compacted.len());
    println!("compacted: {}", to_string_compacted(&compacted));

    let checksum = calc_checksum(compacted);
    println!("checksum: {checksum}");

    Some(checksum)
}

fn to_string_compacted(compacted: &Vec<Block>) -> String {
    compacted
        .iter()
        .map(|b| match b {
            Block::File(id) => id.to_string(),
            Block::Free => ".".to_string(),
        })
        .join("")
}

fn calc_checksum(compacted: Vec<Block>) -> u64 {
    let checksum = compacted
        .iter()
        .enumerate()
        .fold(0, |acc, (index, block)| match block {
            Block::File(id) => acc + (index as u64 * id),
            Block::Free => acc,
        });
    checksum
}

fn parse_layout(input: &str) -> Vec<Block> {
    let mut layout = Vec::new();
    let mut file_id = 0;
    for (index, char) in input.lines().next().unwrap().chars().enumerate() {
        let is_file_block = index % 2 == 0;
        let times = char.to_digit(10).unwrap() as usize;
        for _ in 0..times {
            if is_file_block {
                layout.push(Block::File(file_id))
            } else {
                layout.push(Block::Free)
            }
        }
        if is_file_block {
            file_id += 1;
        }
    }
    layout
}

pub fn part_two(input: &str) -> Option<u64> {
    let layout = parse_layout(input);
    // println!("layout: {:?}", layout);

    let files = parse_whole_files(&layout);
    //println!("files: {:?}", files);

    let mut poppable = files.clone();
    let mut compacted = Vec::new();
    for (rev_index, file) in files.iter().enumerate().rev() {
        // println!("next file: {:?} at {rev_index}", file);
        let f_opt = poppable.windows(2).find(|&window| {
            let f1 = window[0];
            let f2 = window[1];
            if f1.start_pos >= file.start_pos || f2.start_pos >= file.start_pos {
                return false;
            }
            // println!("window: {:?}, {:?}", f1, f2);
            let free_length = f2.start_pos - (f1.start_pos + f1.length);
            free_length >= file.length
        });
        match f_opt {
            Some(window) => {
                let new_file = File {
                    id: file.id,
                    length: file.length,
                    start_pos: window[0].start_pos + window[0].length,
                };
                poppable.remove(rev_index);
                poppable.push(new_file);
                poppable.sort_by_key(|f| f.start_pos);
                compacted.push(new_file);
                // println!("{:?}", poppable);
            }
            None => {
                // println!("not moving");
                compacted.push(*file);
            }
        };
    }
    compacted.sort_by_key(|f| f.start_pos);
    //println!("compacted: {:?}", compacted);

    //let blocks = files_to_blocks(layout, &compacted);
    // println!("blocks: {}", to_string_compacted(&blocks));
    //let checksum = calc_checksum(blocks);
    let checksum = calc_checksum_from_files(compacted);
    println!("checksum: {checksum}");

    // TODO tests are working but for some reason the calculated checksum is 4954347984216 which seems to be incorrect.
    // spent way too much time on this, aborting further debugging...

    Some(checksum)
}

/*fn files_to_blocks(layout: Vec<Block>, compacted: &Vec<File>) -> Vec<Block> {
    let blocks = (0..layout.len())
        .map(|pos| {
            let pos = pos as u32;
            for f in compacted {
                if f.start_pos > pos {
                    return Block::Free;
                }
                if f.start_pos == pos {
                    return Block::File(f.id);
                }

                if f.start_pos < pos && pos < (f.start_pos + f.length) {
                    return Block::File(f.id);
                }
            }
            Block::Free
        })
        .collect_vec();
    blocks
}*/

#[derive(Debug, Clone, Copy)]
struct File {
    id: u64,
    length: u32,
    start_pos: u32,
}

fn parse_whole_files(layout: &Vec<Block>) -> Vec<File> {
    let mut files = Vec::new();
    let mut previous_block = None;
    for (index, block) in layout.iter().enumerate() {
        let index = index as u32;
        match previous_block {
            Some((i, b)) => {
                if *block == b {
                    continue;
                }
                // the block changed
                match b {
                    Block::File(id) => files.push(File {
                        id,
                        length: index - i,
                        start_pos: i,
                    }),
                    Block::Free => {}
                }
                previous_block = Some((index, *block));
            }
            None => {
                previous_block = Some((index, *block));
                continue;
            }
        }
    }
    if let Some((index, block)) = previous_block {
        match block {
            Block::File(id) => files.push(File {
                id,
                length: layout.len() as u32 - index,
                start_pos: index,
            }),
            Block::Free => {}
        }
    }
    files
}

fn calc_checksum_from_files(compacted: Vec<File>) -> u64 {
    let checksum = compacted.iter().fold(0, |mut acc, file| {
        // println!("{:?}", file);
        for i in file.start_pos..(file.start_pos + file.length) {
            acc += i as u64 * file.id;
        }
        acc
    });
    checksum
}
