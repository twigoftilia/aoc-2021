use std::ops::Deref;

use advent_of_code_2021::rows_to_vector;

static DAY_2_INPUT: &str = include_str!(r"../../inputs/day02.txt");

fn main() {
    let v = rows_to_vector(DAY_2_INPUT);
    let d1_1 = solve_first(&v);
    println!("Day 2\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = solve_second(&v);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn solve_first(lines: &[&str]) -> Result<i32, &'static str> {
    let mut h = 0;
    let mut d = 0;

    for row in lines {
        let pair: Vec<&str> = row.split_whitespace().collect();
        let dir = pair.first().unwrap().deref();
        let val: i32 = pair.last().unwrap().deref().parse().unwrap();
        //let xxx = xx.deref();
        match dir as &str {
            "forward" => {
                h += val;
            }
            "down" => {
                d += val;
            }
            "up" => {
                d -= val;
            }
            _ => println!(" XXX _"),
        }
        // println!(" XXX: h={}  d={}  mul={}", h, d, h * d);
    }
    return Ok(h * d);
}

fn solve_second(lines: &[&str]) -> Result<i32, &'static str> {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;

    for row in lines {
        let pair: Vec<&str> = row.split_whitespace().collect();
        let dir = pair.first().unwrap().deref();
        let val: i32 = pair.last().unwrap().deref().parse().unwrap();
        //let xxx = xx.deref();
        match dir as &str {
            "forward" => {
                h += val;
                d += aim * val;
            }
            "down" => {
                aim += val;
            }
            "up" => {
                aim -= val;
            }
            _ => println!(" XXX _"),
        }
        //  println!(" XXX: h={}  d={}  mul={}", h, d, h * d);
    }
    return Ok(h * d);
}

#[cfg(test)]
mod tests {
    use super::*;

    static DAY_2_INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day02-example.txt");

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(&rows_to_vector(DAY_2_INPUT)).unwrap(), 150);
    }
    #[test]
    fn second_solution() {
        assert_eq!(
            solve_second(&rows_to_vector(DAY_2_INPUT)).unwrap(),
            1813664422
        );
    }
    #[test]
    fn example_first() {
        assert_eq!(
            solve_first(&rows_to_vector(DAY_2_INPUT_EXAMPLE)).unwrap(),
            150
        );
    }

    #[test]
    fn example_second() {
        assert_eq!(
            solve_second(&rows_to_vector(DAY_2_INPUT_EXAMPLE)).unwrap(),
            900
        );
    }
}
