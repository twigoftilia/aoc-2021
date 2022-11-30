static INPUT: &str = include_str!(r"../../inputs/day07.txt");
fn main() {
    //  let v = rows_to_vector(INPUT);
    let d1_1 = puzzle_1(INPUT);
    println!("Day 7\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = puzzle_2(INPUT);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn puzzle_1(row: &str) -> Result<i32, &'static str> {
    let v: Vec<i32> = row.split(',').map(|i| i.parse().unwrap()).collect();
    let max = *v.iter().max().unwrap() + 1;

    //  println!(" XXX v {:?} ", v);
    let mut crab_pos = vec![0; max as usize];
    v.iter().for_each(|j| crab_pos[*j as usize] += 1);

    // println!(" XXX {:?} ", crab_pos);

    let mut min_fuel = None;
    for i in 0..=max {
        let fuel: i32 = crab_pos
            .iter()
            .enumerate()
            .map(|crab_pos| {
                let dist = (i as i32 - crab_pos.0 as i32).abs() * crab_pos.1;
                // println!(
                //     " XXX MAP {:?}  dist: {}  crap_pos: {:?} ",
                //     i, dist, crab_pos
                // );

                dist
            })
            .sum();

        if min_fuel.is_none() || fuel < min_fuel.unwrap() {
            min_fuel = Some(fuel);
        }
        if fuel > min_fuel.unwrap() {
            break;
        }
    }

    //  .for_each(|j: u64| fishes_of_age[j as usize] += 1);

    // (0..days).for_each(|_day| {
    //     fishes_of_age[6 + 1] += fishes_of_age[0];
    //     fishes_of_age.rotate_left(1);
    // });

    Ok(min_fuel.unwrap())
}

fn puzzle_2(row: &str) -> Result<i32, &'static str> {
    let v: Vec<i32> = row.split(',').map(|i| i.parse().unwrap()).collect();
    let max = *v.iter().max().unwrap() + 1;

    //  println!(" XXX v {:?} ", v);
    let mut crab_pos = vec![0; max as usize];
    v.iter().for_each(|j| crab_pos[*j as usize] += 1);

    // println!(" XXX {:?} ", crab_pos);

    let mut min_fuel = None;

    for i in 0..=max {
        let fuel: i32 = crab_pos
            .iter()
            .enumerate()
            .map(|crab_pos| {
                let dist = (i as i32 - crab_pos.0 as i32).abs();
                let fuel_for_a_crab = dist * (dist + 1) / 2;
                let tot_fuel = fuel_for_a_crab * crab_pos.1;
                tot_fuel
            })
            .sum();

        if min_fuel.is_none() || fuel < min_fuel.unwrap() {
            min_fuel = Some(fuel);
        }
        // if fuel > min_fuel.unwrap() {
        //     break;
        // }
    }

    Ok(min_fuel.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day07-example.txt");

    #[test]
    fn p1_sol() {
        assert_eq!(puzzle_1(INPUT).unwrap(), 335271);
    }
    #[test]
    fn p2_sol() {
        assert_eq!(puzzle_2(INPUT).unwrap(), 95851339);
    }
    #[test]
    fn p1_ex() {
        assert_eq!(puzzle_1(INPUT_EXAMPLE).unwrap(), 37);
    }

    #[test]
    fn p2_ex() {
        assert_eq!(puzzle_2(INPUT_EXAMPLE).unwrap(), 168);
    }
}
