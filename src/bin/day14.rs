use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!(r"../../inputs/day14.txt");
fn main() {
    //  let v = rows_to_vector(INPUT);
    let d1_1 = puzzle_1_and_2(INPUT, 10);
    println!("Day 14\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = puzzle_1_and_2(INPUT, 40);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn puzzle_1_and_2(rows: &str, steps: i32) -> Result<usize, &'static str> {
    let manual = Manual::from_input(rows);

    let map = manual.resolve(steps);
    let max = map.iter().max_by_key(|(_, v)| *v).map(|(k, v)| v).unwrap();
    let min = map.iter().min_by_key(|(_, v)| *v).map(|(k, v)| v).unwrap();
    Ok(max - min)
}

#[derive(Debug)]
struct Manual {
    template: Vec<char>,
    rules: HashMap<(char, char), Vec<(char, char)>>,
}

impl Manual {
    fn resolve(&self, steps: i32) -> HashMap<char, usize> {
        let mut char_size_map: HashMap<char, usize> = HashMap::new();
        let mut template_pairs: HashMap<(char, char), usize> = HashMap::new();
        for c in 0..(self.template.len() - 1) {
            *template_pairs
                .entry((self.template[c], self.template[c + 1]))
                .or_default() += 1 as usize;
        }

        let mut total_pair_count = template_pairs;

        (0..steps).for_each(|_step| {
            let mut local_pair_count: HashMap<(char, char), usize> = HashMap::new();

            for pair in &total_pair_count {
                self.rules.get(&pair.0).unwrap().iter().for_each(|s| {
                    let v = local_pair_count.entry(*s).or_default();
                    *v += pair.1;
                });
            }
            total_pair_count = local_pair_count;
        });

        for pair in &total_pair_count {
            *char_size_map.entry(pair.0 .0).or_default() += pair.1;
        }

        let s = self.template.last().unwrap();
        *char_size_map.entry(*s).or_default() += 1;

        char_size_map
    }

    fn from_input(input: &str) -> Manual {
        let mut x = Manual {
            template: Vec::new(),
            rules: HashMap::new(),
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
                    vec![
                        (
                            rule[0].chars().nth(0).unwrap(),
                            rule[1].chars().nth(0).unwrap(),
                        ),
                        (
                            rule[1].chars().nth(0).unwrap(),
                            rule[0].chars().nth(1).unwrap(),
                        ),
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
        assert_eq!(puzzle_1_and_2(INPUT, 10).unwrap(), 3587);
    }
    #[test]
    fn p2_solution() {
        assert_eq!(puzzle_1_and_2(INPUT, 40).unwrap(), 3906445077999);
    }
    #[test]
    fn p1_ex() {
        assert_eq!(puzzle_1_and_2(INPUT_EXAMPLE, 10).unwrap(), 1588);
    }
    #[test]
    fn p2_ex() {
        assert_eq!(puzzle_1_and_2(INPUT_EXAMPLE, 40).unwrap(), 2188189693529);
    }
}
