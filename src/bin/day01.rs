use advent_of_code_2021::integer_rows_to_vector;

static DAY_1_INPUT: &str = include_str!(r"../../inputs/day01.txt");

fn main() {
    let v = integer_rows_to_vector(DAY_1_INPUT);
    let d1_1 = solve_first(&v);
    println!("Day 1\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = solve_second(&v, 3);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn solve_first(numbers: &[i32]) -> Result<i32, &'static str> {
    let mut last = 0;
    let mut larger = 0;
    for (i, current) in numbers.iter().enumerate() {
        if i > 0 && *current > last {
            //           println!("XXX  {} {}  {}", i, current, last);
            larger += 1;
        }
        last = *current;
    }
    return Ok(larger);
}

fn solve_second(numbers: &Vec<i32>, window_length: usize) -> Result<i32, &'static str> {
    let mut last_sum = 0;
    let mut larger = 0;
    let first_frame_end_offset = window_length - 1;
    for i in first_frame_end_offset..numbers.len() {
        if i >= first_frame_end_offset {
            let pos_first_in_window = i - first_frame_end_offset;
            let current_sum: i32 = numbers[pos_first_in_window..i + 1].iter().sum();
            if i > window_length && current_sum > last_sum {
                larger += 1;
            }
            last_sum = current_sum;
        }
    }
    return Ok(larger);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_solution() {
        assert_eq!(
            solve_first(&integer_rows_to_vector(DAY_1_INPUT)).unwrap(),
            1121
        );
    }

    #[test]
    fn second_solution() {
        assert_eq!(
            solve_second(&integer_rows_to_vector(DAY_1_INPUT), 3).unwrap(),
            1065
        );
    }
    #[test]
    fn example_first() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve_first(&input).unwrap(), 7);
    }
}
