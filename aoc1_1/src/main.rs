use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt").unwrap();

    let mut max: u32 = 0;
    let mut pc: u32 = 0;
    for line in content.lines() {
        if line == "\n\r" || line.is_empty() {
            if pc > max {
                max = pc;
            }
            pc = 0;
        } else {
            pc += line.parse::<u32>().unwrap();
        }
    }

    print!("{}", max);
}
