use std::collections::HashSet;

static INPUT: &str = include_str!(r"../../inputs/day09.txt");
fn main() {
    //  let v = rows_to_vector(INPUT);
    let d1_1 = puzzle_1(INPUT);
    println!("Day 9\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = puzzle_2(INPUT);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn puzzle_1(rows: &str) -> Result<i32, &'static str> {
    let hf = HF::from_input(rows);
    let mut sum = 0;
    let low_points = hf.get_low_points();
    low_points.iter().for_each(|p| sum += hf.hd[*p] as i32 + 1);
    Ok(sum)
}

fn puzzle_2(rows: &str) -> Result<i32, &'static str> {
    let hf = HF::from_input(rows);
    let low_points = hf.get_low_points();
    let mut basin_sizes = Vec::new();

    low_points.iter().for_each(|p| {
        let bassin = find_basins(&hf, *p, None);
        //     println!("BASSIN LEN for{}={}  {:?}", p, bassin.len(), bassin);
        basin_sizes.push(bassin.len());
    });

    basin_sizes.sort();
    let l = basin_sizes.len();
    let a = basin_sizes[l - 1];
    let b = basin_sizes[l - 2];
    let c = basin_sizes[l - 3];

    Ok((a * b * c) as i32)
}

fn find_basins(hf: &HF, pos: usize, pos_prev: Option<&HashSet<usize>>) -> HashSet<usize> {
    let mut current_basin = HashSet::new();
    if hf.hd[pos] == 9 {
        return current_basin;
    }
    let adj_all = hf.get_adjacent_pos(pos);
    //    println!(" pos={} all {:?}", pos, &adj_all);
    let adj_all_filtered: Vec<usize>;
    if pos_prev.is_none() {
        adj_all_filtered = adj_all;
    } else {
        current_basin.extend(pos_prev.unwrap());
        adj_all_filtered = adj_all
            .iter()
            .map(|s| *s)
            .filter(|ss| {
                let x = pos_prev.unwrap();
                !x.contains(ss)
            })
            .collect();
    }
    current_basin.insert(pos);
    for p in adj_all_filtered {
        current_basin.extend(find_basins(hf, p, Some(&current_basin)));
    }
    return current_basin;
}

#[derive(Debug)]
struct HF {
    hd: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl HF {
    fn from_input(input: &str) -> HF {
        let mut hf = HF {
            hd: Vec::new(),
            cols: 0,
            rows: 0,
        };
        hf.rows = input.lines().count();
        input.lines().enumerate().for_each(|(i, s)| {
            if i == 0 {
                hf.cols = s.len();
            }
            s.chars()
                .for_each(|s| hf.hd.push(s.to_string().parse().unwrap()));
        });
        hf
    }

    fn get_low_points(&self) -> HashSet<usize> {
        let mut lows = HashSet::new();
        let len = self.rows * self.cols;
        (0..len).for_each(|p| {
            let adj = self.get_adjacent_pos(p);
            let h = self.hd[p];
            let adj_min = adj.iter().map(|s| &self.hd[*s]).min().unwrap();
            if *adj_min > h {
                lows.insert(p);
            }
        });
        lows
    }

    fn get_adjacent_pos(&self, pos: usize) -> Vec<usize> {
        let row = pos / self.cols;
        let col = pos % self.cols;
        let mut adj = Vec::new();

        if row > 0 {
            adj.push(pos - self.cols);
        }
        if row < self.rows - 1 {
            adj.push(pos + self.cols);
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
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day09-example.txt");

    #[test]
    fn p1_solution() {
        assert_eq!(puzzle_1(INPUT).unwrap(), 506);
    }
    #[test]
    fn p2_solution() {
        assert_eq!(puzzle_2(INPUT).unwrap(), 931200);
        // 303048  and 317520 t low
    }
    #[test]
    fn p1_ex() {
        assert_eq!(puzzle_1(INPUT_EXAMPLE).unwrap(), 15);
    }
    #[test]
    fn p2_ex() {
        assert_eq!(puzzle_2(INPUT_EXAMPLE).unwrap(), 1134);
    }
}
