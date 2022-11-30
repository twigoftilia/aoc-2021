use std::collections::HashMap;

use advent_of_code_2021::rows_to_vector;

static INPUT: &str = include_str!(r"../../inputs/day05.txt");
fn main() {
    let v = rows_to_vector(INPUT);
    let d1_1 = solve_first(&v);
    println!("Day 5\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = solve_second(&v);
    println!(" second puzzle: {}", d1_2.unwrap());
}

type Point = (i32, i32);

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn is_h_or_v(&self) -> bool {
        self.p1.0 == self.p2.0 || self.p1.1 == self.p2.1
    }

    fn points_in_straigt_lines(&self) -> Vec<Point> {
        let mut points = Vec::new();

        // if self.p1.1 == self.p2.1 {
        //     let x_min = self.p1.0.min(self.p2.0);
        //     for i in x_min..self.p1.0.max(self.p2.0) + 1 {
        //         points.push((i, self.p2.1));
        //     }
        // } else if self.p1.0 == self.p2.0 {
        //     let y_min = self.p1.1.min(self.p2.1);
        //     for i in y_min..self.p1.1.max(self.p2.1) + 1 {
        //         points.push((self.p1.0, i));
        //     }
        // }

        let x_delta = if self.p1.0 < self.p2.0 {
            1
        } else if self.p1.0 > self.p2.0 {
            -1
        } else {
            0
        };
        let y_delta = if self.p1.1 < self.p2.1 {
            1
        } else if self.p1.1 > self.p2.1 {
            -1
        } else {
            0
        };

        let len = (self.p1.0 - self.p2.0)
            .abs()
            .max((self.p1.1 - self.p2.1).abs())
            + 1;

        // println!(
        //     "XXX self {:?}\n  l={} xd={} yd={} ",
        //     self, len, x_delta, y_delta
        // );

        for i in 0..len {
            // println!(
            //     "   i={} x={} y={} ",
            //     i,
            //     self.p1.0 + i * x_delta,
            //     self.p1.1 + i * y_delta
            // );

            points.push((self.p1.0 + i * x_delta, self.p1.1 + i * y_delta));
        }

        points
    }
}

fn parse_input(rows: &[&str]) -> Vec<Line> {
    let mut lines = Vec::new();
    let tst: Vec<i32> = rows
        .iter()
        .flat_map(|s| s.split(" -> "))
        .flat_map(|s| s.split(","))
        .map(|s| s.trim().parse().expect("Not an integer"))
        .collect();

    let mut inx = 0;
    while inx + 4 <= tst.len() {
        let l = Line {
            p1: (tst[inx + 0], tst[inx + 1]),
            p2: (tst[inx + 2], tst[inx + 3]),
        };
        // if l.is_h_or_v() {
        lines.push(l);
        // }
        inx += 4;
    }
    // println!("XXXX {:?}", tst);
    lines
}

fn solve_first(rows: &[&str]) -> Result<i32, &'static str> {
    let l = parse_input(rows);
    let lines: Vec<&Line> = l.iter().filter(|l| l.is_h_or_v()).collect();
    //println!("returned LINES {:?}", lines);
    let mut pointmap = HashMap::new();
    for line in lines.into_iter() {
        let points = line.points_in_straigt_lines();
        for p in points {
            *pointmap.entry(p).or_insert(0) += 1;
        }
    }
    let hotspots = pointmap.values().filter(|p| **p > 1).count();
    Ok(hotspots as i32)
}

fn solve_second(rows: &[&str]) -> Result<i32, &'static str> {
    let lines: Vec<Line> = parse_input(rows);
    //println!("returned LINES {:?}", lines);
    let mut pointmap = HashMap::new();
    for line in lines.into_iter() {
        let points = line.points_in_straigt_lines();
        for p in points {
            *pointmap.entry(p).or_insert(0) += 1;
        }
    }
    let hotspots = pointmap.values().filter(|p| **p > 1).count();
    Ok(hotspots as i32)

    //panic!("Failure is not an option");
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day05-example.txt");

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(&rows_to_vector(INPUT)).unwrap(), 4655);
    }
    #[test]
    fn second_solution() {
        assert_eq!(solve_second(&rows_to_vector(INPUT)).unwrap(), 4590);
    }
    #[test]
    fn example_first() {
        assert_eq!(solve_first(&rows_to_vector(INPUT_EXAMPLE)).unwrap(), 5);
    }

    #[test]
    fn example_second() {
        assert_eq!(solve_second(&rows_to_vector(INPUT_EXAMPLE)).unwrap(), 12);
    }
}
