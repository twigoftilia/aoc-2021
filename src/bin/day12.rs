use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!(r"../../inputs/day12.txt");
fn main() {
    //  let v = rows_to_vector(INPUT);
    let d1_1 = puzzle_1(INPUT);
    println!("Day 12\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = puzzle_2(INPUT);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn puzzle_1(rows: &str) -> Result<usize, &'static str> {
    puzzle_common(rows, false)
}

fn puzzle_2(rows: &str) -> Result<usize, &'static str> {
    puzzle_common(rows, true)
}

fn puzzle_common(rows: &str, small_allow_twice_for_one: bool) -> Result<usize, &'static str> {
    let raw_pairs: HashSet<(&str, &str)> = rows
        .lines()
        .map(|s| s.split_at(s.find("-").unwrap()))
        .flat_map(|(key, val)| [(key, &val[1..]), (&val[1..], key)])
        .collect();

    let mut rooms_adj: HashMap<&str, HashSet<&str>> = HashMap::new();

    raw_pairs.iter().for_each(|(k, v)| {
        rooms_adj.entry(k).or_insert(HashSet::new()).insert(v);
    });

    //  println!("XXX: {:?}", rooms_adj);
    let visited = HashMap::new();
    let number_of_routes = routes(
        &rooms_adj,
        "start",
        &visited,
        true,
        small_allow_twice_for_one,
        false,
    );
    Ok(number_of_routes)
}

fn routes(
    rooms_adj: &HashMap<&str, HashSet<&str>>,
    visit_room: &str,
    visited: &HashMap<&str, i32>,
    is_small: bool,
    small_allow_twice: bool,
    small_used_twice: bool,
) -> usize {
    let mut routes_to_end = 0;
    let mut visited_updated = visited.clone();
    let mut small_used_twice_local = small_used_twice;

    if is_small {
        *visited_updated.entry(visit_room).or_insert(0) += 1;

        if visit_room == "end" {
            routes_to_end += 1;
            return routes_to_end;
        }

        if small_allow_twice && *visited_updated.get(visit_room).unwrap() >= 2 {
            small_used_twice_local = true;
        }
    }

    rooms_adj
        .get(visit_room)
        .iter()
        .flat_map(|s| s.iter())
        .for_each(|adj_room| {
            let is_lowercase = adj_room.to_lowercase() == *adj_room;
            if is_lowercase {
                if *adj_room == "start" {
                    return;
                }
                let prev_visits_to_adj = visited.get(*adj_room);
                let prev_visits_to_adj = *prev_visits_to_adj.unwrap_or(&0);
                if ((!small_allow_twice || small_used_twice_local) && prev_visits_to_adj >= 1)
                    || (small_allow_twice && prev_visits_to_adj >= 2)
                {
                    return;
                }
            }

            routes_to_end += routes(
                rooms_adj,
                adj_room,
                &visited_updated,
                is_lowercase,
                small_allow_twice,
                small_used_twice_local,
            );
        });

    routes_to_end
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day12-example.txt");
    static INPUT_EXAMPLE_2: &str = include_str!(r"../../inputs/day12-example-2.txt");

    #[test]
    fn p1_solution() {
        assert_eq!(puzzle_1(INPUT).unwrap(), 4304);
    }
    #[test]
    fn p2_solution() {
        assert_eq!(puzzle_2(INPUT).unwrap(), 118242);
    }
    #[test]
    fn p1_ex() {
        assert_eq!(puzzle_1(INPUT_EXAMPLE).unwrap(), 10);
    }
    #[test]
    fn p1_ex2() {
        assert_eq!(puzzle_1(INPUT_EXAMPLE_2).unwrap(), 19);
    }
    #[test]
    fn p2_ex() {
        assert_eq!(puzzle_2(INPUT_EXAMPLE).unwrap(), 36);
    }
    #[test]
    fn p2_ex2() {
        assert_eq!(puzzle_2(INPUT_EXAMPLE_2).unwrap(), 103);
    }
}
