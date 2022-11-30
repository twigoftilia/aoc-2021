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
    let visited = HashSet::new();
    let number_of_routes = routes(&rooms_adj, "start", &visited);
    Ok(number_of_routes)
}

fn routes(
    rooms_adj: &HashMap<&str, HashSet<&str>>,
    visit_room: &str,
    visited: &HashSet<&str>,
) -> usize {
    println!(
        "> : enter visit_room {}-{:?}    {:?}",
        visit_room,
        rooms_adj.get(visit_room),
        visited
    );

    let mut routes_to_end = 0;

    let mut visited_updated = visited.clone();
    visited_updated.insert(visit_room);

    if visit_room == "end" {
        routes_to_end += 1;
        return routes_to_end;
    }

    // if visit_room.to_lowercase() == visit_room {
    //     visited.
    // } else {
    // }

    rooms_adj
        .get(visit_room)
        .iter()
        .flat_map(|s| s.iter())
        .for_each(|adj_room| {
            // println!(
            //     "WWWWWW=   {} {} ",
            //     visit_room.to_lowercase().eq(visit_room),
            //     visited.contains(visit_room)
            // );

            if !((adj_room.to_lowercase() == *adj_room) && visited.contains(adj_room)) {
                //           println!(" XXX visiting adj {} from {}", adj_room, visit_room);

                routes_to_end += routes(rooms_adj, adj_room, &visited_updated);
            }
        });

    routes_to_end
}

fn puzzle_2(rows: &str) -> Result<usize, &'static str> {
    Ok(0)
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
        assert_eq!(puzzle_2(INPUT).unwrap(), 0);
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
        assert_eq!(puzzle_2(INPUT_EXAMPLE).unwrap(), 103);
    }
    103
}
