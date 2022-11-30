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
    let len = hf.rows * hf.cols;
    (0..len).for_each(|p| {
        // println!(" PP: {} ", p);
        let s = hf.get_adjacent(p);
        let h = hf.hd[p];
        if s.iter()
            .filter(|d| d.is_some())
            .map(|d| d.unwrap())
            .min()
            .unwrap()
            > h
        {
            println!(" LOW POINT:  {} {:?}", h, s);
            sum += h as i32 + 1;
        }
        // println!(" S: {:?} ", s);
    });
    Ok(sum)
}

fn puzzle_2(rows: &str) -> Result<i32, &'static str> {
    let hf = HF::from_input(rows);
    let sum = 0;
    let mut low_points = Vec::new();
    let len = hf.rows * hf.cols;
    let mut basin_sizes = Vec::new();

    (0..len).for_each(|p| {
        // println!(" PP: {} ", p);
        let s = hf.get_adjacent(p);
        let h = hf.hd[p];

        if s.iter()
            .filter(|d| d.is_some())
            .map(|d| d.unwrap())
            .min()
            .unwrap()
            > h
        {
            low_points.push(p);
        }
    });

    low_points = vec![1];

    println!("LOWS {:?}   ", low_points);
    low_points.iter().for_each(|p| {
        let mut bassin = Vec::new();
        let mut start = HashSet::new();
        start.insert(*p);
        find_bass(&hf, &start, &mut bassin);
        basin_sizes.push(bassin.len());

        println!("BASSIN LEN for{}={}  {:?}", p, bassin.len(), bassin);
    });

    // for s in basin_sizes {
    //     println!(" SiZE: {}", s);
    // }

    Ok(sum)
}

/*
2199943210
3987894921
9856789892
8767896789
9899965678
*/
fn find_bass(hf: &HF, new_pos: &HashSet<usize>) -> Vec<usize> {
    let mut next_iters_pos = HashSet::new();

    println!("ENTRY  POSs={:?}  BAS={:?}", new_pos, bassin);

    for posp in new_pos {
        let pos = *posp;
        let c_depth = hf.hd[pos];
        let ad_poss = hf.get_adjacent_pos(pos);

        let ad_poss_not_in_bass: Vec<usize> = ad_poss
            .iter()
            .filter(|p| {
                let v = bassin.contains(*p);
                if v {
                    println!("   >>  CHECK {} {} contaied in {:?}  ", *p, v, bassin);
                }
                !v
            })
            .map(|s| *s)
            .collect();
        println!(
            "   Adjs all={:?}  filtered: {:?}",
            ad_poss, ad_poss_not_in_bass
        );

        if ad_poss_not_in_bass.is_empty() {
            continue;
        }

        let min = ad_poss_not_in_bass.iter().map(|s| hf.hd[*s]).min();
        // println!(" DDDDDDDD POS={}  MIN ad {:?}    ", pos, min);

        if min.is_none() {
            println!(
                " >>>>>>>>>>>>>>>>>   POS={} to BASSIN  {:?}    ",
                pos, bassin
            );
        }

        if min.unwrap() > c_depth {
            bassin.push(pos);

            for p in ad_poss_not_in_bass {
                next_iters_pos.insert(p);
            }
            println!(
                "--- added POS={} to {:?}   next-{:?}",
                pos, bassin, &next_iters_pos
            );
        }

        if !next_iters_pos.is_empty() {
            let r = find_bass(hf, &next_iters_pos, bassin);
            if !r {
                panic!("SS")
            }
        }
    }
    true
    //   new_pos
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

    fn get_adjacent(&self, pos: usize) -> Vec<Option<u8>> {
        let row = pos / self.cols;
        let col = pos % self.cols;
        // println!("XXX POS: {} row: {} col: {}", pos, row, col);
        //println!("ROW: {} ", row);
        //let col = pos - row * self.cols;
        self.get_adjacent_2(col, row)
    }

    /* return oreder: over, under, left, right
     */
    fn get_adjacent_2(&self, col: usize, row: usize) -> Vec<Option<u8>> {
        let pos = row * self.cols + col;

        let mut adj = Vec::new();

        if row > 0 {
            adj.push(Some(self.hd[pos - self.cols]));
        } else {
            adj.push(None);
        }

        if row < self.rows - 1 {
            // println!(
            //     "Get under... pos {} {} {} {} ",
            //     pos,
            //     col,
            //     row,
            //     pos + self.cols
            // );
            adj.push(Some(self.hd[pos + self.cols]));
        } else {
            adj.push(None);
        }

        if col > 0 {
            adj.push(Some(self.hd[pos - 1]));
        } else {
            adj.push(None);
        }

        if col < self.cols - 1 {
            adj.push(Some(self.hd[pos + 1]));
        } else {
            adj.push(None);
        }
        adj
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
        assert_eq!(puzzle_2(INPUT).unwrap(), 0);
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
// println!(
//     "Get under... pos {} {} {} {} ",
//     pos,
//     col,
//     row,
//     pos + self.cols
// );
