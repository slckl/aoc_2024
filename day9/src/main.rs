use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    File { id: u32, size: u32 },
    Free(u32),
}

fn parse(i: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut file = true;
    let mut file_idx = 0;
    for ch in i.chars() {
        let num: u32 = ch.to_digit(10).unwrap();
        if file {
            blocks.push(Block::File {
                size: num,
                id: file_idx,
            });
            file_idx += 1;
        } else {
            blocks.push(Block::Free(num));
        }
        file = !file;
    }
    blocks
}

#[test]
fn test_parse() {
    let mem = parse("12345");
    assert_eq!(mem.len(), 5);
    assert_eq!(mem[0], Block::File { id: 0, size: 1 });
    assert_eq!(mem[1], Block::Free(2));
    assert_eq!(mem[2], Block::File { id: 1, size: 3 });
    assert_eq!(mem[3], Block::Free(4));
    assert_eq!(mem[4], Block::File { id: 2, size: 5 });
}

#[derive(Debug, Clone, Copy)]
enum ExpandedBlock {
    File(u32),
    Free,
}

fn expand(mem: &[Block]) -> Vec<ExpandedBlock> {
    let mut exp = Vec::new();
    for block in mem {
        match block {
            Block::File { id, size } => {
                for _file_blocks in 0..*size {
                    exp.push(ExpandedBlock::File(*id));
                }
            }
            Block::Free(size) => {
                for _free_blocks in 0..*size {
                    exp.push(ExpandedBlock::Free);
                }
            }
        }
    }

    exp
}

#[cfg(test)]
fn to_string(blocks: &[ExpandedBlock]) -> String {
    let mut s = String::with_capacity(blocks.len());
    for b in blocks {
        match b {
            ExpandedBlock::File(id) => s.push_str(&id.to_string()),
            ExpandedBlock::Free => s.push('.'),
        }
    }
    s
}

#[test]
fn test_expand() {
    let mem = parse("12345");
    let expanded = expand(&mem);
    let printed = to_string(&expanded);
    assert_eq!(printed, "0..111....22222");

    let mem = parse("2333133121414131402");
    let expanded = expand(&mem);
    let printed = to_string(&expanded);
    assert_eq!(printed, "00...111...2...333.44.5555.6666.777.888899");
}

fn compact(mut mem: Vec<ExpandedBlock>) -> Vec<ExpandedBlock> {
    let mut fwd_idx = 0;
    let mut rev_idx = mem.len() - 1;
    while rev_idx > fwd_idx {
        // Go back until you hit first block with a file id.
        while let ExpandedBlock::Free = &mem[rev_idx] {
            rev_idx -= 1;
        }
        let ExpandedBlock::File(_file_id) = &mem[rev_idx] else {
            panic!();
        };
        // Go forward until you hit the first free block.
        while let ExpandedBlock::File(_) = &mem[fwd_idx] {
            fwd_idx += 1;
        }
        // Swap fwd_idx block with rwd_idx block.
        mem[fwd_idx] = mem[rev_idx];
        mem[rev_idx] = ExpandedBlock::Free;

        fwd_idx += 1;
        rev_idx -= 1;
    }
    mem
}

#[test]
fn test_compact() {
    let mem = parse("12345");
    let expanded = expand(&mem);
    let compacted = compact(expanded);
    let printed = to_string(&compacted);
    assert_eq!(printed, "022111222......");

    let mem = parse("2333133121414131402");
    let expanded = expand(&mem);
    let compacted = compact(expanded);
    let printed = to_string(&compacted);
    assert_eq!(printed, "0099811188827773336446555566..............");
}

fn checksum(exp: &[ExpandedBlock]) -> u64 {
    let mut sum = 0u64;
    for (block_idx, block) in exp.iter().enumerate() {
        if let ExpandedBlock::File(file_id) = block {
            sum += (block_idx as u32 * file_id) as u64;
        }
    }
    sum
}

#[test]
fn test_checksum() {
    let mem = parse("2333133121414131402");
    let expanded = expand(&mem);
    let compacted = compact(expanded);
    let checksum = checksum(&compacted);
    assert_eq!(checksum, 1928);
}

/// Moves whole files, instead of just blocks...
fn compact_p2(mem: Vec<Block>) -> Vec<Block> {
    // This nonsense is a bit easier, if we rev the whole thing.
    let mut mem: Vec<_> = mem.into_iter().rev().collect();
    // println!("{}", to_string(&expand(&mem).into_iter().rev().collect::<Vec<_>>()));
    for idx in 0..mem.len() {
        // println!("Fwd: {idx}: {:?}", mem[idx]);
        let Block::File { id, size } = mem[idx] else {
            continue;
        };
        // We got a file, yay. Try to shove it in the first place in the back.
        for rev_idx in (idx..mem.len() - 1).rev() {
            // println!("Rev: {rev_idx}: {:?}", mem[rev_idx]);
            let Block::Free(free) = mem[rev_idx] else {
                continue;
            };
            if free >= size {
                // Yay, it fits here.
                mem[rev_idx] = Block::File { id, size };
                // mem.insert(rev_idx, Block::File { id, size });
                mem[idx] = Block::Free(size);
                let remaining_free = free - size;
                mem.insert(rev_idx, Block::Free(remaining_free));
                // println!("{}", to_string(&expand(&mem).into_iter().rev().collect::<Vec<_>>()));
                break;
            }
        }
    }
    // Reverse memory back.
    mem.into_iter().rev().collect()
}

#[test]
fn test_compact_p2() {
    let mem = parse("2333133121414131402");
    assert_eq!(
        to_string(&expand(&mem)),
        "00...111...2...333.44.5555.6666.777.888899"
    );
    let mem = compact_p2(mem);
    let mem = expand(&mem);
    let printed = to_string(&mem);
    assert_eq!(printed, "00992111777.44.333....5555.6666.....8888..");
    let checksum = checksum(&mem);
    assert_eq!(checksum, 2858);
}

fn main() {
    let input = read_to_string("day9/input.txt").unwrap();
    let mem = parse(&input);
    let mem = expand(&mem);
    let mem = compact(mem);
    println!("p1: {}", checksum(&mem));

    let mem = parse(&input);
    let mem = compact_p2(mem);
    let mem = expand(&mem);
    println!("p2: {}", checksum(&mem));
}
