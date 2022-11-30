use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!(r"../../inputs/day10.txt");
fn main() {
    let (p1, p2) = puzzle_1_and_2(INPUT).unwrap();
    println!("Day 10\n first puzzle: {}", p1);
    println!(" second puzzle: {}", p2);
}

fn puzzle_1_and_2(rows: &str) -> Result<(usize, usize), &'static str> {
    let closes_map = HashMap::from([(')', '('), ('}', '{'), (']', '['), ('>', '<')]);
    let score_1_map = HashMap::from([(')', 3), ('}', 57), (']', 1197), ('>', 25137)]);
    let score_2_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut score_1_sum: usize = 0;
    let mut score_2_val: Vec<usize> = Vec::new();

    let openers: HashSet<&char> = closes_map.values().collect();

    'row: for s in rows.lines() {
        let mut score_2_line_val = 0;
        let mut stack = Vec::new();

        for current_char in s.chars() {
            if openers.contains(&current_char) {
                stack.push(current_char);
            } else {
                let popped_char_opt = stack.pop();

                if let Some(popped_char) = popped_char_opt {
                    if let Some(c) = closes_map.get(&current_char) {
                        if popped_char == *c {
                            continue;
                        } else {
                            // bad char, score, and go to next line
                            score_1_sum += score_1_map.get(&current_char).unwrap();
                            continue 'row;
                        }
                    }
                }
            }
        }

        let l = stack.len();
        for i in 0..l {
            let c = stack[l - 1 - i];
            score_2_line_val *= 5;
            score_2_line_val += score_2_map[&c];
        }
        score_2_val.push(score_2_line_val);
    }
    score_2_val.sort();
    Ok((score_1_sum, score_2_val[score_2_val.len() / 2]))
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day10-example.txt");

    #[test]
    fn p1_solution() {
        let (p1, p2) = puzzle_1_and_2(INPUT).unwrap();
        assert_eq!(p1, 168417);
    }
    #[test]
    fn p2_solution() {
        let (p1, p2) = puzzle_1_and_2(INPUT).unwrap();
        assert_eq!(p2, 2802519786);
    }
    #[test]
    fn p1_ex() {
        let (p1, p2) = puzzle_1_and_2(INPUT_EXAMPLE).unwrap();
        assert_eq!(p1, 26397);
    }
    #[test]
    fn p2_ex() {
        let (p1, p2) = puzzle_1_and_2(INPUT_EXAMPLE).unwrap();
        assert_eq!(p2, 288957);
    }
}
