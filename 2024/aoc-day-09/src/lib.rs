use std::array::from_fn;
use std::collections::VecDeque;

pub fn part_one_v1(input: &str) -> u64 {
    let mut file_blocks = VecDeque::with_capacity(10_000);
    let mut free_blocks = VecDeque::with_capacity(10_000);

    for (idx, &ch) in input.trim().as_bytes().iter().enumerate() {
        let blocks = (ch - b'0') as u32;
        if idx % 2 != 0 {
            free_blocks.push_back(blocks);
        } else {
            file_blocks.push_back((idx / 2, blocks));
        }
    }

    let mut checksum = 0;
    let mut idx = 0;
    let mut next_file = true;

    loop {
        if next_file {
            let Some((file_id, file_block)) = file_blocks.pop_front() else {
                break;
            };

            checksum +=
                file_id as u64 * file_block as u64 * (idx + idx + file_block - 1) as u64 / 2;
            idx += file_block;
        }

        if let Some(free_block) = free_blocks.pop_front() {
            let Some((file_id, file_block)) = file_blocks.pop_back() else {
                break;
            };

            let blocks = file_block.min(free_block);
            checksum += file_id as u64 * blocks as u64 * (idx + idx + blocks - 1) as u64 / 2;
            idx += blocks;

            if free_block > file_block {
                next_file = false;
                free_blocks.push_front(free_block - file_block);
            } else {
                next_file = true;
                if file_block > free_block {
                    file_blocks.push_back((file_id, file_block - free_block));
                }
            }
        }
    }

    checksum
}

pub fn part_one_v2(input: &str) -> u64 {
    let input = &input.trim_ascii_end().as_bytes();

    let mut checksum = 0;
    let mut p = 0;
    let mut l = 0;
    let mut r = if input.len() % 2 == 0 {
        input.len() - 2
    } else {
        input.len() - 1
    };

    let mut free_blocks_available = 0;
    let mut file_blocks_remaining = 0;

    while l < r {
        if l % 2 == 0 {
            // L points to a file
            // It will not move, so add it to the checksum
            let blocks = (input[l] - b'0') as u64;
            let file_id = (l / 2) as u64;

            checksum += file_id * blocks * (p + p + blocks - 1) / 2;

            p += blocks;
            l += 1;
        } else {
            // L points to a free blocks descriptor
            if free_blocks_available == 0 {
                free_blocks_available = (input[l] - b'0') as u64;
            }

            if file_blocks_remaining == 0 {
                file_blocks_remaining = (input[r] - b'0') as u64
            }

            let blocks = free_blocks_available.min(file_blocks_remaining);
            free_blocks_available -= blocks;
            file_blocks_remaining -= blocks;

            let file_id = (r / 2) as u64;
            checksum += file_id * blocks * (p + p + blocks - 1) / 2;

            p += blocks;
            l += (free_blocks_available == 0) as usize * 1;
            r -= (file_blocks_remaining == 0) as usize * 2;
        }
    }

    // Last chunk when L == R corner-case
    if file_blocks_remaining != 0 {
        let file_id = (r / 2) as u64;
        checksum += file_id * file_blocks_remaining * (p + p + file_blocks_remaining - 1) / 2;
    } else if l == r && l % 2 == 0 {
        let file_id = (l / 2) as u64;
        let blocks = (input[l] - b'0') as u64;
        checksum += file_id * blocks * (p + p + blocks - 1) / 2;
    }

    checksum
}

pub fn part_two_v1(input: &str) -> u64 {
    let input = input.trim_ascii_end().as_bytes();

    // Note: we can eliminate the file blocks vector, if we do
    // a second pass over the input in reverse, then we can will be
    // able to calculate the checksum as we parse, similar to part1/v2
    let mut file_blocks = Vec::with_capacity(10_000);
    let mut free_blocks: [VecDeque<u32>; 10] = from_fn(|_| VecDeque::new());
    for idx in 1..free_blocks.len() {
        free_blocks[idx].reserve(136 * 8);
    }

    let mut start_pos = 0u32;
    for (idx, &ch) in input.iter().enumerate() {
        let blocks = (ch - b'0') as u32;

        if idx % 2 != 0 {
            free_blocks[blocks as usize].push_back(start_pos);
        } else {
            file_blocks.push(((idx / 2) as u32, start_pos, blocks));
        }

        start_pos += blocks;
    }

    let mut checksum = 0;
    while let Some((file_id, file_start_idx, file_blocks)) = file_blocks.pop() {
        let mut selected_start_idx = file_start_idx;
        let mut selected_block_size = 0;

        for block_size in file_blocks..free_blocks.len() as u32 {
            let free_list = &free_blocks[block_size as usize];
            if let Some(start_idx) = free_list.front().copied() {
                if start_idx < selected_start_idx {
                    selected_start_idx = start_idx;
                    selected_block_size = block_size;
                }
            }
        }

        let mut new_file_start = file_start_idx;
        if selected_block_size > 0 {
            new_file_start = selected_start_idx;
            free_blocks[selected_block_size as usize].pop_front();

            let remaining_blocks = selected_block_size - file_blocks;
            if remaining_blocks > 0 {
                let remaining_start_idx = selected_start_idx + file_blocks;
                let free_list = &mut free_blocks[remaining_blocks as usize];

                let insert_idx = free_list
                    .binary_search(&remaining_start_idx)
                    .unwrap_or_else(|e| e);
                free_list.insert(insert_idx, remaining_start_idx);
            }
        }

        let to_add = file_id as u64
            * file_blocks as u64
            * (new_file_start + new_file_start + file_blocks - 1) as u64
            / 2;
        checksum += to_add;
    }

    checksum
}

pub fn part_two_v2(input: &str) -> u64 {
    let input = input.trim_ascii_end().as_bytes();

    let mut free_blocks: [VecDeque<u32>; 10] = from_fn(|_| VecDeque::new());
    for idx in 1..free_blocks.len() {
        free_blocks[idx].reserve(136 * 8);
    }

    let mut start_pos = 0u32;
    for (idx, &ch) in input.iter().enumerate() {
        let blocks = (ch - b'0') as u32;

        if idx % 2 != 0 {
            free_blocks[blocks as usize].push_back(start_pos);
        }

        start_pos += blocks;
    }

    let mut checksum = 0;
    for (idx, &ch) in input.iter().enumerate().rev() {
        let file_blocks = (ch - b'0') as u32;

        // the `start_pos` has the total number of blocks,
        // so when we go in reverse and subtract the number
        // of file/empty blocks, we get the start position
        start_pos -= file_blocks;

        // skip free blocks
        if idx % 2 != 0 {
            continue;
        }

        let file_id = (idx / 2) as u64;
        let mut selected_start_idx = start_pos;
        let mut selected_block_size = 0;

        for block_size in file_blocks..free_blocks.len() as u32 {
            let free_list = &free_blocks[block_size as usize];
            if let Some(start_idx) = free_list.front().copied() {
                if start_idx < selected_start_idx {
                    selected_start_idx = start_idx;
                    selected_block_size = block_size;
                }
            }
        }

        let mut new_file_start = start_pos;
        if selected_block_size > 0 {
            new_file_start = selected_start_idx;
            free_blocks[selected_block_size as usize].pop_front();

            let remaining_blocks = selected_block_size - file_blocks;
            if remaining_blocks > 0 {
                let remaining_start_idx = selected_start_idx + file_blocks;
                let free_list = &mut free_blocks[remaining_blocks as usize];

                let insert_idx = free_list
                    .binary_search(&remaining_start_idx)
                    .unwrap_or_else(|e| e);
                free_list.insert(insert_idx, remaining_start_idx);
            }
        }

        checksum += file_id
            * file_blocks as u64
            * (new_file_start + new_file_start + file_blocks - 1) as u64
            / 2;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v1(&input);
        assert_eq!(6_299_243_228_569, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v2(&input);
        assert_eq!(6_299_243_228_569, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v1(&input);
        assert_eq!(6326952672104, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v2(&input);
        assert_eq!(6326952672104, answer);
    }
}
