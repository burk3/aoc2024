use std::collections::BTreeMap;

#[derive(Clone, PartialEq)]
enum Block {
    File(u64),
    Empty,
}
struct File {
    id: u64,
    start: usize,
    size: usize,
}
impl File {
    fn end(&self) -> usize {
        self.start + self.size
    }
    fn last_block(&self) -> usize {
        self.end() - 1
    }
}

fn print_blocks(blocks: &Vec<Block>) {
    for block in blocks {
        match block {
            Block::File(id) => print!("{}", id),
            Block::Empty => print!("_"),
        }
    }
    println!();
}


fn main() {
    let input = include_str!("input.txt");
    let mut blocks: Vec<Block> = Vec::new();
    let mut input_iter = input.chars();
    let mut block_id = 0;
    // i want to keep track of files and gaps for part 2
    let mut files: Vec<File> = Vec::new();
    let mut empties: BTreeMap<usize, usize> = BTreeMap::new();
    loop {
        if let Some(c) = input_iter.next() {
            let size = c.to_digit(10).unwrap() as usize;
            files.push(File { id: block_id, start: blocks.len(), size });
            for _ in 0..size {
                blocks.push(Block::File(block_id));
            }
            block_id += 1;
        } else {
            break;
        }

        if let Some(c) = input_iter.next() {
            let size = c.to_digit(10).unwrap() as usize;
            if size > 0 {
                empties.insert(blocks.len(), size);
            }
            for _ in 0..size {
                blocks.push(Block::Empty);
            }
        } else {
            break;
        }
    }
    // for part 2
    let mut blocks2 = blocks.clone();

    // do the part one compaction (aka fragmentation)
    let mut right_cursor: usize = blocks.len() - 1;
    let mut left_cursor: usize = 0;
    loop {
        while blocks[right_cursor] == Block::Empty {
            right_cursor -= 1;
        }
        while blocks[left_cursor] != Block::Empty {
            left_cursor += 1;
        }
        if left_cursor >= right_cursor {
            break;
        }
        blocks.swap(left_cursor, right_cursor);
    }

    // do the "checksum"
    let mut checksum: u64 = 0;
    for (i, block) in blocks.iter().enumerate() {
        if let Block::File(id) = block {
            checksum += i as u64 * id;
        }
    }
    println!("Checksum: {}", checksum);

    // part 2
    for file in files.iter().rev() {
        if let Some((empty_start, empty_size)) = find_good_empty(&empties, file.size) {
            if file.start < empty_start {
                continue;
            }
            let empty_end = empty_start + empty_size;
            if empty_size == file.size {
                // fill the empty
                for i in empty_start..empty_end {
                    blocks2[i] = Block::File(file.id);
                }
                empties.remove(&empty_start);
            } else {
                // fill the empty partially
                for i in empty_start..empty_start + file.size {
                    blocks2[i] = Block::File(file.id);
                }
                empties.remove(&empty_start);
                empties.insert(empty_start + file.size, empty_size - file.size);
            }
            for i in file.start..file.end() {
                blocks2[i] = Block::Empty;
            }
        }
    }
    // do the "checksum"
    let mut checksum: u64 = 0;
    for (i, block) in blocks2.iter().enumerate() {
        if let Block::File(id) = block {
            checksum += i as u64 * id;
        }
    }
    println!("Checksum: {}", checksum);
}

fn find_good_empty(empties: &BTreeMap<usize, usize>, size: usize) -> Option<(usize, usize)> {
    for (start, empty_size) in empties.iter() {
        if *empty_size >= size {
            return Some((*start, *empty_size));
        }
    }
    None
}