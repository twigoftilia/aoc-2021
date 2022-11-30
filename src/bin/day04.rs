use advent_of_code_2021::rows_to_vector;

static INPUT: &str = include_str!(r"../../inputs/day04.txt");
fn main() {
    let v = rows_to_vector(INPUT);
    let d1_1 = solve_first(&v);
    println!("Day 4\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = solve_second(&v);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn bingo_check(brick: &Vec<i32>, numbers: &[i32]) -> bool {
    //   println!("XXX checking numbers: {:?}\n brick: {:?} ", numbers, brick);
    for a in 0..5 {
        let mut x_axis_invalidated = false;
        let mut y_axis_invalidated = false;

        for b in 0..5 {
            if x_axis_invalidated && y_axis_invalidated {
                break;
            }
            // check horizontal, a = y, b=x
            let char = brick.get(a * 5 + b).unwrap();
            if !x_axis_invalidated && !numbers.contains(char) {
                x_axis_invalidated = true;
            }
            // check vertical, a = x, b=y
            let char = brick.get(a + b * 5).unwrap();
            if !y_axis_invalidated && !numbers.contains(char) {
                y_axis_invalidated = true;
            }
        }
        if !x_axis_invalidated || !y_axis_invalidated {
            return true;
        }
    }
    false
}

fn parse_input(lines: &[&str]) -> (Vec<i32>, Vec<Vec<i32>>) {
    let row: Vec<i32> = lines[0]
        .split(",")
        .map(|line| line.trim().parse().expect("Not an integer"))
        .collect();
    let mut brick_offset = 2;
    let mut bricks = Vec::new();

    while brick_offset + 4 <= lines.len() {
        let brickpos = &lines[brick_offset..];
        let br: Vec<i32> = brickpos
            .into_iter()
            .take(5)
            .map(|x| *x)
            .flat_map(|s| s.split(' '))
            .filter(|s| s.len() > 0)
            .map(|s| s.trim().parse().expect("Not an integer"))
            .collect();
        bricks.push(br);
        brick_offset += 6;
    }
    (row, bricks)
}

fn solve_first(lines: &[&str]) -> Result<i32, &'static str> {
    let (row, bricks) = parse_input(lines);
    //    println!("returned ROW {:?}", row);
    for no_of_balls_drawn in 5..row.len() {
        let drawn = &row[..no_of_balls_drawn];

        let matched_bricks: Vec<&Vec<i32>> = bricks
            .iter()
            .filter(|brick| bingo_check(&brick, drawn))
            .take(1)
            .collect();

        if matched_bricks.len() > 0 {
            //           println!("Matched: {:?} \ndrawn_numbers {:?}", matched_bricks, drawn);
            let winner_brick = matched_bricks.get(0).unwrap();
            let a: i32 = winner_brick.iter().filter(|s| !drawn.contains(*s)).sum(); // .collect();

            let sum = a * drawn.last().unwrap();
            //          println!("SUM {:?}*{}={}", a, drawn.last().unwrap(), sum);
            return Ok(sum);
        }
    }
    panic!("Failure is not an option");
}

fn solve_second(lines: &[&str]) -> Result<i32, &'static str> {
    let (row, bricks) = parse_input(lines);

    for no_of_drawn_to_remove in 0..row.len() {
        let drawn = &row[..(row.len() - no_of_drawn_to_remove)];
        let matched_bricks: Vec<&Vec<i32>> = bricks
            .iter()
            .filter(|brick| !bingo_check(&brick, &drawn))
            .take(1)
            .collect();

        if matched_bricks.len() > 0 {
            //           println!("Matched: {:?} \ndrawn_numbers {:?}", matched_bricks, drawn);
            let last_winner_brick = matched_bricks.get(0).unwrap();
            let winner_drawn = &row[..(drawn.len() + 1)];
            //            println!("winner_drawn: {:?}", winner_drawn);

            let a: i32 = last_winner_brick
                .iter()
                .filter(|s| !winner_drawn.contains(*s))
                .sum(); // .collect();

            let last_winning_bingo_number = winner_drawn.last().unwrap();
            let sum = a * last_winning_bingo_number;
            //           println!("SUM {:?}*{}={}", a, last_winning_bingo_number, sum);
            return Ok(sum);
        }
    }
    panic!("Failure is not an option");
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day04-example.txt");

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(&rows_to_vector(INPUT)).unwrap(), 8442);
    }
    #[test]
    fn second_solution() {
        assert_eq!(solve_second(&rows_to_vector(INPUT)).unwrap(), 4590);
    }
    #[test]
    fn example_first() {
        assert_eq!(solve_first(&rows_to_vector(INPUT_EXAMPLE)).unwrap(), 4512);
    }

    #[test]
    fn example_second() {
        assert_eq!(solve_second(&rows_to_vector(INPUT_EXAMPLE)).unwrap(), 1924);
    }
}
