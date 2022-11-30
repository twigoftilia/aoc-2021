use std::panic;

use advent_of_code_2021::rows_to_vector;

static DAY_3_INPUT: &str = include_str!(r"../../inputs/day03.txt");
static BITS: usize = 12;

fn main() {
    let v = rows_to_vector(DAY_3_INPUT);
    let d1_1 = solve_first(&v, BITS);
    println!("Day 3\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = solve_second(&v, BITS);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn solve_first(lines: &[&str], bits: usize) -> Result<i32, &'static str> {
    let mut count = vec![0; bits];
    lines.iter().enumerate().for_each(|(_i, current)| {
        let intval = isize::from_str_radix(current, 2).unwrap() as u32;

        for j in 0..bits as u32 {
            if intval & (1_u32 << j) > 0 {
                count[j as usize] += 1;
            }
        }
    });

    let len = lines.len();
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for j in 0..bits {
        //  println!(" Count:  {}  {} ", j, count[j]);
        if count[j] > len as u32 / 2 {
            gamma_rate += 1 << j;
        } else {
            epsilon_rate += 1 << j;
        }
    }
    let consumption = gamma_rate * epsilon_rate;
    // println!(" XXX: {} {} {}", gamma_rate, epsilon_rate, consumption);
    return Ok(consumption);
}

fn solve_second(lines: &[&str], bits: usize) -> Result<i32, &'static str> {
    let ints: Vec<u32> = lines
        .into_iter()
        .map(|line| isize::from_str_radix(line, 2).unwrap() as u32)
        .collect();

    let (_, rating) = second_loop(true, &ints, bits);
    let (_, co2) = second_loop(false, &ints, bits);
    return Ok((rating * co2) as i32);
}

fn second_loop(ox: bool, remainers: &Vec<u32>, bits: usize) -> (usize, u32) {
    let mut rems = Vec::new();

    for bit_c in 0..bits {
        let bit = bits - bit_c - 1;
        let mut one_pos = Vec::new();
        let mut zero_pos = Vec::new();

        let remainers_to_check: &Vec<u32>;
        if bit_c == 0 {
            remainers_to_check = remainers;
        } else {
            remainers_to_check = &rems;
        }

        remainers_to_check
            .iter()
            .enumerate()
            .for_each(|(i, current)| {
                if current & (1_u32 << bit) > 0 {
                    one_pos.push(i);
                } else {
                    zero_pos.push(i);
                };
            });

        let keepers =
            if ox && zero_pos.len() > one_pos.len() || !ox && one_pos.len() >= zero_pos.len() {
                one_pos.iter().map(|i| remainers_to_check[*i]).collect()
            } else {
                zero_pos.iter().map(|i| remainers_to_check[*i]).collect()
            };

        // println!(
        //     " XXX: {}\n   zeros: {:?} {}\n   ones:  {:?} {},",
        //     bit,
        //     zero_pos,
        //     zero_pos.len(),
        //     one_pos,
        //     one_pos.len(),
        // );

        rems.clear();
        rems = keepers;

        if rems.len() <= 1 {
            break;
        }
    }

    if rems.len() > 1 {
        return second_loop(ox, &rems, bits);
    } else if rems.len() == 0 {
        panic!("Failure is not an option");
    }

    (rems.len(), rems[0])
}

/*
0 0100
1 1110
1 0110
1 0111
1 0101
0 1111
0 0111
1 1100
1 0000
1 1001
0 0010
0 1010

0 1 111
0 1 010

0 1 010
-----------------------
0 0100
1 1110
1 0110
1 0111
1 0101
0 1111
0 0111
1 1100
1 0000
1 1001
0 0010
0 1010

1 1 110
1 0 110
1 0 111
1 0 101
1 1 100
1 0 000
1 1 001

10 1 10
10 1 11
10 1 01
10 0 00

101 1 0
101 1 1
101 0 1

1011 0
1011 1

1011 1
*/

#[cfg(test)]
mod tests {
    use super::*;
    static BITS_EXAMPLE: usize = 5;
    static DAY_3_INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day03-example.txt");

    #[test]
    fn first_solution() {
        assert_eq!(
            solve_first(&rows_to_vector(DAY_3_INPUT), BITS).unwrap(),
            3549854
        );
    }
    #[test]
    fn second_solution() {
        assert_eq!(
            solve_second(&rows_to_vector(DAY_3_INPUT), BITS).unwrap(),
            3765399
        );
    }
    #[test]
    fn example_first() {
        assert_eq!(
            solve_first(&rows_to_vector(DAY_3_INPUT_EXAMPLE), BITS_EXAMPLE).unwrap(),
            198
        );
    }

    #[test]
    fn example_second() {
        assert_eq!(
            solve_second(&rows_to_vector(DAY_3_INPUT_EXAMPLE), BITS_EXAMPLE).unwrap(),
            230
        );
    }
}
