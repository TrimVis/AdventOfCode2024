use std::fs;

// const INPUT_FILE: &str = "inputs/day9.test";
const INPUT_FILE: &str = "inputs/day9.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

type BlockSize = u64;
type Uid = usize;

#[derive(Debug)]
enum Block {
    Free(BlockSize),
    File(BlockSize, Uid),
}

pub fn solve_p1() -> usize {
    let _content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut disk: Vec<Block> = _content
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let size = c.to_string().parse().expect("Expected a number");
            if i % 2 == 0 {
                Block::File(size, i / 2)
            } else {
                Block::Free(size)
            }
        })
        .collect();

    // Defragment the disk
    let mut first_free = 1;
    let mut free_size = 0;
    'o: loop {
        // println!("Disk: {:?}", disk);
        let last_block = disk.pop();
        if last_block.is_none() {
            break 'o;
        }
        let last_block = last_block.unwrap();

        if let Block::Free(size) = last_block {
            // println!("{:?}: Last block is free, remembering", last_block);
            free_size += size;
            continue;
        } else if let Block::File(last_block_size, last_block_id) = last_block {
            // println!("{:?}: Last block is file, moving to front", last_block);
            free_size += last_block_size;
            let mut last_block_size: i128 = last_block_size.into();
            for i in first_free..disk.len() {
                let cur_block = disk.get_mut(i);
                if cur_block.is_none() {
                    break 'o;
                }
                let cur_block = cur_block.unwrap();
                match cur_block {
                    Block::Free(size) => {
                        // println!("{:?}: Found free space of size {}", last_block, size);
                        last_block_size = last_block_size - (*size) as i128;
                        let new_block_size: u64 = if last_block_size > 0 {
                            *size
                        } else {
                            ((*size) as i128 + last_block_size) as u64
                        };
                        *cur_block = Block::File(new_block_size, last_block_id);
                    }
                    Block::File(_, _) => {
                        continue;
                    }
                }

                if last_block_size < 0 {
                    disk.insert(
                        i + 1,
                        Block::Free(last_block_size.abs().try_into().unwrap()),
                    );
                }
                if last_block_size <= 0 {
                    first_free = i + 1;
                    continue 'o;
                }
            }
            // println!("Could not find position for last block!!!");
            disk.push(Block::File(last_block_size as u64, last_block_id));
            free_size -= last_block_size as u64;
            break 'o;
        }
    }
    disk.push(Block::Free(free_size));

    // println!("Disk: {:?}", disk);

    // Calculate checksum
    let mut checksum = 0;
    let mut checksum_idx = 0;
    for block in disk {
        if let Block::File(size, uid) = block {
            for i in checksum_idx..(checksum_idx + size) {
                checksum += (i as usize) * uid;
            }
            checksum_idx += size;
        }
    }

    checksum
}

pub fn solve_p2() -> usize {
    let _content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");
    0
}
