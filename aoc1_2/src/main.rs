use std::{fs};

fn main() {
    let content = fs::read_to_string("in.txt").unwrap();

    let mut max = vec![];
    let mut pc: u32 = 0;

    for line in content.lines() {
        if line == "\n\r" || line.is_empty() {
            max.push(pc);
            pc = 0;
        } else {
            pc += line.parse::<u32>().unwrap();
        }
    }

    max.sort();
    max.reverse();
    print!("{}\n\r", max[0] + max[1] + max[2]);
}
