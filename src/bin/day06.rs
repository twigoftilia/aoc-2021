static INPUT: &str = include_str!(r"../../inputs/day06.txt");
fn main() {
    //  let v = rows_to_vector(INPUT);
    let d1_1 = solve(INPUT, 80);
    println!("Day 6\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = solve(INPUT, 256);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn solve(row: &str, days: i32) -> Result<u64, &'static str> {
    let mut fishes_of_age = vec![0; 9];
    row.split(',')
        .map(|i| i.parse().unwrap())
        .for_each(|j: u64| fishes_of_age[j as usize] += 1);

    (0..days).for_each(|_day| {
        fishes_of_age[6 + 1] += fishes_of_age[0];
        fishes_of_age.rotate_left(1);
    });

    Ok(fishes_of_age.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day06-example.txt");

    #[test]
    fn first_solution() {
        assert_eq!(solve(INPUT, 80).unwrap(), 390923);
    }
    #[test]
    fn second_solution() {
        assert_eq!(solve(INPUT, 256).unwrap(), 1749945484935);
    }
    #[test]
    fn example_first() {
        assert_eq!(solve(INPUT_EXAMPLE, 18).unwrap(), 26);
    }
    #[test]
    fn example_first2() {
        assert_eq!(solve(INPUT_EXAMPLE, 80).unwrap(), 5934);
    }

    #[test]
    fn example_second() {
        assert_eq!(solve(INPUT_EXAMPLE, 256).unwrap(), 26984457539);
    }
}
