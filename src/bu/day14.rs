use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!(r"../../inputs/day14.txt");
fn main() {
    //  let v = rows_to_vector(INPUT);
    let d1_1 = puzzle_1(INPUT, 10);
    println!("Day 14\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = puzzle_1(INPUT, 40);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn puzzle_1(rows: &str, steps: i32) -> Result<usize, &'static str> {
    let manual = Manual::from_input(rows);

    let chars = manual.resolve(steps);

    let mut m: HashMap<char, usize> = HashMap::new();
    let _ = chars.iter().for_each(|c| {
        *m.entry(*c).or_default() += 1;
    });

    let max = m.iter().max_by_key(|(_, v)| *v).map(|(k, v)| v).unwrap();
    let min = m.iter().min_by_key(|(_, v)| *v).map(|(k, v)| v).unwrap();

    //   println!("XX {:?}   {:?} {:?}", chars, max, min);
    Ok(max - min)
}

fn puzzle_2(rows: &str) -> Result<usize, &'static str> {
    let manual = Manual::from_input(rows);
    Ok(0)
}

#[derive(Debug)]
struct Manual {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
    rules2: HashMap<(char, char), [char; 3]>,
}

impl Manual {
    fn resolve(&self, steps: i32) -> Vec<char> {
        // 4 7 13

        let mut resolved = self.template.clone();
        for step in 0..steps {
            //      println!("XX step {:?}   resolved : {:?} ", step, resolved);
            let mut current = Vec::new();
            for c in 0..(resolved.len() - 1) {
                let resolved_char = self.rules2.get(&(resolved[c + 0], resolved[c + 1]));
                // println!(
                //     "XXX step {:?}   c : {:?}     res: {:?}",
                //     step, c, resolved_char
                // );
                if c == 0 {
                    current.push(resolved[c + 0]);
                }
                if let Some(resolved_char) = resolved_char {
                    current.push(*resolved_char);
                }
                current.push(resolved[c + 1]);
            }
            resolved = current;
        }
        resolved
    }

    fn from_input(input: &str) -> Manual {
        let mut x = Manual {
            template: Vec::new(),
            rules: HashMap::new(),
            rules2: HashMap::new(),
        };
        input.lines().enumerate().for_each(|(i, s)| {
            if i == 0 {
                s.chars().for_each(|c| x.template.push(c));
                return;
            }
            let rule: Vec<&str> = s.split(" -> ").collect();
            if rule.len() == 2 {
                x.rules.insert(
                    (
                        rule[0].chars().nth(0).unwrap(),
                        rule[0].chars().nth(1).unwrap(),
                    ),
                    rule[1].chars().nth(0).unwrap(),
                );

                x.rules2.insert(
                    (
                        rule[0].chars().nth(0).unwrap(),
                        rule[0].chars().nth(1).unwrap(),
                    ),
                    [
                        rule[0].chars().nth(0).unwrap(),
                        rule[1].chars().nth(0).unwrap(),
                        rule[0].chars().nth(1).unwrap(),
                    ],
                );
            }
        });
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day14-example.txt");

    #[test]
    fn p1_solution() {
        assert_eq!(puzzle_1(INPUT, 10).unwrap(), 3587);
    }
    #[test]
    fn p2_solution() {
        assert_eq!(puzzle_1(INPUT, 40).unwrap(), 40);
        // 303048  and 317520 t low
    }
    #[test]
    fn p1_ex() {
        assert_eq!(puzzle_1(INPUT_EXAMPLE, 10).unwrap(), 3587);
    }
    #[test]
    fn p2_ex() {
        let start = 2;

        let mut last = start;
        for i in 0..40 {
            // 2. 3 , 5, 9  17
            let next = last + last - 1;
            println!("{} {} {}", i, last, next);
            last = next;
        }

        // assert_eq!(puzzle_1(INPUT_EXAMPLE, 40).unwrap(), 2188189693529);
    }
}
