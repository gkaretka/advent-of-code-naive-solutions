use std::fs;
use std::collections::{HashSet, HashMap};

fn main() {
    let content = fs::read_to_string("in.txt").unwrap();

    let nscore1 = naive_soltuion(&content);
    print!("Naive approach score1: {}\n\r", nscore1);

    let nscore2 = naive_second_solution(&content);
    print!("Naive approach score2: {}\n\r", nscore2);
}

fn naive_second_solution(content: &str) -> u32 {
    let mut score: u32 = 0;

    let mut lines_iter = content.lines().peekable();
    while lines_iter.peek().is_some() {
        let lines = [lines_iter.next().unwrap(), lines_iter.next().unwrap(), lines_iter.next().unwrap()];
        let mut items: HashMap<u8, u8> = HashMap::<u8, u8>::new();

        for line in lines {
            let mut ct_items: HashSet<u8> = HashSet::<u8>::new();
            for ck in line.bytes() {
                let cc = get_char_val(ck);
                if !ct_items.contains(&cc) {
                    let prev_cnt = match items.get(&cc) {
                        Some(val) => {val.to_owned()},
                        None => 0 as u8,
                    };
                    ct_items.insert(cc);
                    items.insert(cc, prev_cnt + 1);
                }
            }
        }

        for key in items {
            print!("key {} val {}\n\r", key.0, key.1);
            if key.1 == 3 {
                score += key.0 as u32;
            }
        }
    }

    score
}


fn naive_soltuion(content: &str) -> u32 {
    let mut score: u32 = 0;

    for line in content.lines() {
        let mut items1 = HashSet::<u8>::new();
        let mut items2 = HashSet::<u8>::new();

        let line_length = line.len();
        for i in 0..line_length / 2 {
            let c_char_1 = get_char_val(line.as_bytes()[i]);
            let c_char_2 = get_char_val(line.as_bytes()[line_length / 2 + i]);

            if items2.contains(&c_char_1) {
                score += c_char_1 as u32;
                break;
            }

            items1.insert(c_char_1);
            
            if items1.contains(&c_char_2) {
                score += c_char_2 as u32;
                break;
            }

            items2.insert(c_char_2);
        }
    }
    score
}

const UPPER_CASE_A_Z_START: u8 = 27;
const UPPER_CASE_A_Z_RANGE_LOW: u8 = 65;
const UPPER_CASE_A_Z_RANGE_HEIGH: u8 = 90;

const LOWER_CASE_A_Z_START: u8 = 1;
const LOWER_CASE_A_Z_RANGE_LOW: u8 = 97;

fn get_char_val(ch: u8) -> u8 {
    if ch >= UPPER_CASE_A_Z_RANGE_LOW && ch <= UPPER_CASE_A_Z_RANGE_HEIGH {
        ch - UPPER_CASE_A_Z_RANGE_LOW + UPPER_CASE_A_Z_START
    } else {
        ch - LOWER_CASE_A_Z_RANGE_LOW + LOWER_CASE_A_Z_START
    }
}