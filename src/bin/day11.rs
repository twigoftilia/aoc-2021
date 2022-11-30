use std::collections::HashSet;

static INPUT: &str = include_str!(r"../../inputs/day11.txt");
fn main() {
    //  let v = rows_to_vector(INPUT);
    let d1_1 = puzzle_1(INPUT);
    println!("Day 11\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = puzzle_2(INPUT);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn puzzle_1(rows: &str) -> Result<usize, &'static str> {
    let dumbo_grid = OctopusGrid::from_input(rows);
    Ok(dumbo_grid.step(100))
}

fn puzzle_2(rows: &str) -> Result<usize, &'static str> {
    let dumbo_grid = OctopusGrid::from_input(rows);
    Ok(dumbo_grid.step_p2())
}

#[derive(Debug)]
struct OctopusGrid {
    dumbo_strengths: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl OctopusGrid {
    fn from_input(input: &str) -> OctopusGrid {
        let mut hf = OctopusGrid {
            dumbo_strengths: Vec::new(),
            cols: 0,
            rows: 0,
        };
        hf.rows = input.lines().count();
        input.lines().enumerate().for_each(|(i, s)| {
            if i == 0 {
                hf.cols = s.len();
            }
            s.chars()
                .for_each(|s| hf.dumbo_strengths.push(s.to_string().parse().unwrap()));
        });
        hf
    }

    fn step(mut self, steps: i32) -> usize {
        let mut flashes = 0;
        //      println!("XXX self: {:?}", &self);

        (0..steps).for_each(|_| {
            let mut flashed_dumbos = HashSet::new();

            (0..self.dumbo_strengths.len()).for_each(|pos| {
                self.dumbo_strengths[pos] += 1;
            });

            loop {
                let mut flashed_in_iter = false;
                (0..self.dumbo_strengths.len()).for_each(|pos| {
                    let strength = self.dumbo_strengths[pos];
                    //                   println!("XXX checking dumbo at pos: {:?} - s={}", pos, strength);

                    if strength > 9 && !flashed_dumbos.contains(&pos) {
                        // flash! ah ah aaa
                        flashed_in_iter = true;
                        self.dumbo_strengths[pos] = 0;
                        flashed_dumbos.insert(pos);

                        self.get_adjacent_pos(pos)
                            .into_iter()
                            .filter(|p| !flashed_dumbos.contains(p))
                            .for_each(|i| {
                                self.dumbo_strengths[i] += 1;
                            });
                    }
                });
                if !flashed_in_iter {
                    break;
                }
            }
            flashes += flashed_dumbos.len();
        });

        flashes
    }

    // Ok, just get over it, won´t refactor with p1
    fn step_p2(mut self) -> usize {
        let mut step = 0;
        //      println!("XXX self: {:?}", &self);

        loop {
            step += 1;
            let mut flashed_dumbos = HashSet::new();

            (0..self.dumbo_strengths.len()).for_each(|pos| {
                self.dumbo_strengths[pos] += 1;
            });

            loop {
                let mut flashed_in_iter = false;
                (0..self.dumbo_strengths.len()).for_each(|pos| {
                    let strength = self.dumbo_strengths[pos];
                    //                   println!("XXX checking dumbo at pos: {:?} - s={}", pos, strength);

                    if strength > 9 && !flashed_dumbos.contains(&pos) {
                        // flash! ah ah aaa
                        flashed_in_iter = true;
                        self.dumbo_strengths[pos] = 0;
                        flashed_dumbos.insert(pos);

                        self.get_adjacent_pos(pos)
                            .into_iter()
                            .filter(|p| !flashed_dumbos.contains(p))
                            .for_each(|i| {
                                self.dumbo_strengths[i] += 1;
                            });
                    }
                });
                if !flashed_in_iter {
                    break;
                }
            }

            if flashed_dumbos.len() == self.dumbo_strengths.len() {
                break;
            }

            //            println!("FLASJEDS step = {}  {:?}", step, flashed_dumbos.len());
        }

        step
    }

    fn get_adjacent_pos(&self, pos: usize) -> Vec<usize> {
        let row = pos / self.cols;
        let col = pos % self.cols;
        let mut adj = Vec::new();

        if row > 0 {
            adj.push(pos - self.cols);
            if col > 0 {
                adj.push(pos - self.cols - 1);
            }
            if col < self.cols - 1 {
                adj.push(pos - self.cols + 1);
            }
        }
        if row < self.rows - 1 {
            adj.push(pos + self.cols);
            if col > 0 {
                adj.push(pos + self.cols - 1);
            }
            if col < self.cols - 1 {
                adj.push(pos + self.cols + 1);
            }
        }
        if col > 0 {
            adj.push(pos - 1);
        }
        if col < self.cols - 1 {
            adj.push(pos + 1);
        }
        adj
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day11-example.txt");

    #[test]
    fn p1_solution() {
        assert_eq!(puzzle_1(INPUT).unwrap(), 1615);
    }
    #[test]
    fn p2_solution() {
        assert_eq!(puzzle_2(INPUT).unwrap(), 249);
        // 303048  and 317520 t low
    }
    #[test]
    fn p1_ex() {
        assert_eq!(puzzle_1(INPUT_EXAMPLE).unwrap(), 1656);
    }
    #[test]
    fn p2_ex() {
        assert_eq!(puzzle_2(INPUT_EXAMPLE).unwrap(), 195);
    }
}
