use std::collections::{HashMap, HashSet};

extern crate nalgebra as na;

static INPUT: &str = include_str!(r"../../inputs/day13.txt");
fn main() {
    //  let v = rows_to_vector(INPUT);
    let d1_1 = puzzle_1(INPUT);
    println!("Day 13\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = puzzle_2(INPUT);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn puzzle_1(rows: &str) -> Result<usize, &'static str> {
    let mut paper = Paper::from_input(rows);
    //   println!("XXX in paper: {:?}", paper);

    paper.fold_once();

    Ok(paper.dots.len())
}

fn puzzle_2(rows: &str) -> Result<usize, &'static str> {
    let mut paper = Paper::from_input(rows);
    paper.fold_all();

    paper.print();

    Ok(paper.dots.len())
}

#[derive(Debug)]
struct Paper {
    dots: HashSet<(u32, u32)>,
    // cols: usize,
    // rows: usize,
    fold_stack: Vec<(bool, u32)>, // (is x , fold line/col)
}

impl Paper {
    fn empty() -> Paper {
        Paper {
            dots: HashSet::new(),
            // cols: 0,
            // rows: 0,
            fold_stack: Vec::new(),
        }
    }

    fn from_input(input: &str) -> Paper {
        let mut paper = Paper::empty();
        input.lines().for_each(|s| {
            let v: Vec<&str> = s.split(',').collect();
            if v.len() == 2 {
                paper
                    .dots
                    .insert((v[0].parse().unwrap(), v[1].parse().unwrap()));
            } else {
                let v: Vec<&str> = s.split('=').collect();
                if v.len() == 2 {
                    paper
                        .fold_stack
                        .insert(0, (v[0].ends_with("x"), v[1].parse().unwrap()));
                }
            }
        });
        paper
    }

    fn fold_all(&mut self) {
        while !self.fold_stack.is_empty() {
            self.fold_once();
        }
    }

    fn fold_once(&mut self) {
        if self.fold_stack.is_empty() {
            return;
        }
        let fold = self.fold_stack.pop().unwrap();
        let fold_pos = fold.1;
        let mut new_dots = self.dots.clone();
        for dot in &self.dots {
            if !fold.0 {
                // fold x (from left)
                let y = dot.1;
                if y > fold_pos {
                    let new_y = y - 2 * (y - fold_pos);
                    new_dots.remove(dot);
                    new_dots.insert((dot.0, new_y));
                }
            } else {
                let x = dot.0;
                if x > fold_pos {
                    let new_x = x - 2 * (x - fold_pos);
                    new_dots.remove(dot);
                    new_dots.insert((new_x, dot.1));
                }
            }
        }
        self.dots = new_dots;
    }

    fn print(&self) {
        // let rows = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;

        for dot in &self.dots {
            cols = cols.max(dot.0 + 1);
            rows = rows.max(dot.1 + 1);
        }
        // println!("DOSTS: {:?} ", self.dots);
        // println!("ROWS: {} ", rows);

        for row in 0..rows {
            print!(" ");
            for col in 0..cols {
                if self.dots.contains(&(col, row)) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day13-example.txt");

    #[test]
    fn p1_solution() {
        assert_eq!(puzzle_1(INPUT).unwrap(), 802);
    }
    #[test]
    fn p2_solution() {
        // XXX  X  X X  X XXXX XXXX  XX  X  X XXX
        // X  X X X  X  X X       X X  X X  X X  X
        // X  X XX   XXXX XXX    X  X    X  X XXX
        // XXX  X X  X  X X     X   X XX X  X X  X
        // X X  X X  X  X X    X    X  X X  X X  X
        // X  X X  X X  X X    XXXX  XXX  XX  XXX
        assert_eq!(puzzle_2(INPUT).unwrap(), 103);
    }

    #[test]
    fn p1_ex() {
        assert_eq!(puzzle_1(INPUT_EXAMPLE).unwrap(), 17);
    }
    #[test]
    fn p2_ex() {
        // XXXXX
        // X   X
        // X   X
        // X   X
        // XXXXX

        assert_eq!(puzzle_2(INPUT_EXAMPLE).unwrap(), 16);
    }
}
