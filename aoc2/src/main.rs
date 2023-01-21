use std::fs;

/*
 * rock     1
 * paper    2
 * scisors  3
 * 
 * lose     0
 * draw     3
 * win      6
 */

fn main() {
    let content = fs::read_to_string("in.txt").expect("Wrong path or file is not readable.");
    let mut score : i32 = 0;

    for line in content.lines() {
        let op = (line.chars().nth(0).unwrap() as i32) - ('A' as i32);
        let me = (line.chars().nth(2).unwrap() as i32) - ('X' as i32);
        
        let _score = if (op + 1) % 3 == me { 6 } else { if op == me { 3 } else { 0 } };
        score += _score + me + 1;
    }
    print!("{}\n\r", score);

    let new_score = second_task(&content.to_owned());
    print!("new score: {}\n\r", new_score);
}

/*
 * X - lose
 * Y - draw
 * Z - win
*/

fn second_task(content: &str) -> i32 {

    let mut score: i32 = 0;
    for line in content.lines() {
        let op = (line.chars().nth(0).unwrap() as i32) - ('A' as i32);
        let to_do = (line.chars().nth(2).unwrap() as i32) - ('X' as i32);

        if to_do == 0 { // lose
            score += (op + 2) % 3 + 1;
        } else if to_do == 1 { // draw
            score += op + 1;
            score += 3;
        } else { // win
            score += (op + 1) % 3 + 1;
            score += 6;
        }
    }

    score
}