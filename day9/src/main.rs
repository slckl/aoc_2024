use std::fs::read_to_string;

#[derive(Debug)]
struct Memory {
    files: Vec<u32>,
    free_blocks: Vec<u32>,
}

fn parse(i: &str) -> Memory {
    let mut files = Vec::new();
    let mut free_blocks = Vec::new();
    let mut file = true;
    for ch in i.chars() {
        let num: u32 = ch.to_digit(10).unwrap();
        if file {
            files.push(num);
        } else {
            free_blocks.push(num);
        }
        file = !file;
    }
    // Last file may omit the free blocks repr.
    if files.len() > free_blocks.len() {
        free_blocks.push(0);
    }
    Memory { files, free_blocks }
}

#[test]
fn test_parse() {
    let mem = parse("12345");
    assert_eq!(mem.files.len(), 3);
    assert_eq!(mem.files[0], 1);
    assert_eq!(mem.free_blocks[0], 2);
    assert_eq!(mem.files[1], 3);
    assert_eq!(mem.free_blocks[1], 4);
    assert_eq!(mem.files[2], 5);
    assert_eq!(mem.free_blocks[2], 0);
}

#[derive(Debug, Clone, Copy)]
enum ExpandedBlock {
    File(u32),
    Free,
}

fn expand(mem: &Memory) -> Vec<ExpandedBlock> {
    let mut exp = Vec::new();
    for file_id in 0..mem.files.len() {
        for _file_blocks in 0..mem.files[file_id] {
            exp.push(ExpandedBlock::File(file_id as u32));
        }
        for _free_blocks in 0..mem.free_blocks[file_id] {
            exp.push(ExpandedBlock::Free);
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
    let mut file_idx = 0;
    for block in exp {
        if let ExpandedBlock::File(file_id) = block {
            sum += (file_idx as u32 * file_id) as u64;
            file_idx += 1;
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

fn main() {
    let input = read_to_string("day9/input.txt").unwrap();
    let mem = parse(&input);
    let mem = expand(&mem);
    let mem = compact(mem);
    println!("p1: {}", checksum(&mem));
}
