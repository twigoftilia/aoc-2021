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
        if s.iter().min().unwrap() > &h {
            println!(" LOW POINT:  {} {:?}", h, s);
            sum += h as i32 + 1;
        }

        // println!(" S: {:?} ", s);
    });

    Ok(sum)
}

fn puzzle_2(rows: &str) -> Result<i32, &'static str> {
    Ok(0)
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

    fn get_adjacent(&self, pos: usize) -> Vec<u8> {
        let row = pos / self.cols;
        let col = pos % self.cols;

        // println!("XXX POS: {} row: {} col: {}", pos, row, col);
        //println!("ROW: {} ", row);

        //let col = pos - row * self.cols;

        self.get_adjacent_2(col, row)
    }

    fn get_adjacent_2(&self, col: usize, row: usize) -> Vec<u8> {
        let pos = row * self.cols + col;

        let mut adj = Vec::new();

        if row > 0 {
            adj.push(self.hd[pos - self.cols]);
        }

        if row < self.rows - 1 {
            // println!(
            //     "Get under... pos {} {} {} {} ",
            //     pos,
            //     col,
            //     row,
            //     pos + self.cols
            // );

            adj.push(self.hd[pos + self.cols]);
        }

        if col > 0 {
            adj.push(self.hd[pos - 1]);
        }

        if col < self.cols - 1 {
            adj.push(self.hd[pos + 1]);
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
        assert_eq!(puzzle_1(INPUT).unwrap(), 0);
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
        assert_eq!(puzzle_2(INPUT_EXAMPLE).unwrap(), 0);
    }
}
