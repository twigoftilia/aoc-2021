use std::collections::HashMap;

static INPUT: &str = include_str!(r"../../inputs/day08.txt");
fn main() {
    //  let v = rows_to_vector(INPUT);
    let d1_1 = puzzle_1(INPUT);
    println!("Day 8\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = puzzle_2(INPUT);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn puzzle_1(rows: &str) -> Result<i32, &'static str> {
    let mut count = 0;
    rows.lines().for_each(|line| {
        let segs: Vec<&str> = line.split('|').collect(); //.split(' ').collect(); //.unw iter().split(' ').collect();
        let segs2: Vec<&str> = segs[1].split(' ').collect();
        let c = segs2
            .iter()
            .map(|s| s.len())
            .filter(|s| *s == 2 || *s == 3 || *s == 4 || *s == 7)
            .count();
        count += c;
    });
    Ok(count as i32)
}

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

 5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
*/

fn puzzle_2(rows: &str) -> Result<i32, &'static str> {
    let mut result = 0;

    rows.lines().for_each(|line| {
        let segs: Vec<&str> = line.split('|').collect(); //.split(' ').collect(); //.unw iter().split(' ').collect();
        let display_patterns: Vec<u8> = segs[0]
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| disp_charencoded_to_bitncoded(s))
            .collect();

        let mut resolved_numbers: HashMap<u8, u8> = HashMap::new();
        let mut unresolved_of235_with_5 = Vec::new();
        let mut unresolved_of069_with_6 = Vec::new();

        display_patterns.iter().for_each(|s| {
            match s.count_ones() {
                2 => {
                    resolved_numbers.insert(1, *s);
                }
                3 => {
                    resolved_numbers.insert(7, *s);
                }
                4 => {
                    resolved_numbers.insert(4, *s);
                }
                5 => unresolved_of235_with_5.push(*s),
                6 => unresolved_of069_with_6.push(*s),
                7 => {
                    resolved_numbers.insert(8, *s);
                }
                _ => {
                    panic!("baaad input")
                }
            };
        });

        //   println!(" XXX resolved_numbers {:?}", resolved_numbers);
        // resolve 5 segs
        // 5  (2)  = cdefg     1av2be   2av3bde 3av4bceg
        // 5  (3)  = bcdef     2av2be   3av3bde 3av4bceg
        // 5  (5)  = abcdf     1av2be   3av3bde 2av4bceg
        // the 3 only 5seg with full "1"
        let x = unresolved_of235_with_5
            .iter()
            .find(|s| {
                let the1 = *resolved_numbers.get(&1).unwrap();
                **s & the1 == the1
            })
            .unwrap();
        resolved_numbers.insert(3, *x);
        // the 2 - only 5seg with exact 2 of for seg overlapping with 4
        let x = unresolved_of235_with_5
            .iter()
            .find(|s| {
                let the4 = *resolved_numbers.get(&4).unwrap();
                (**s & the4).count_ones() == 2
            })
            .unwrap();
        resolved_numbers.insert(2, *x);
        // the 5
        let x = unresolved_of235_with_5
            .iter()
            .find(|s| {
                **s != *resolved_numbers.get(&3).unwrap()
                    && **s != *resolved_numbers.get(&2).unwrap()
            })
            .unwrap();
        resolved_numbers.insert(5, *x);

        // resolve 6  segs
        // 6 innefattar 1/2  1:a
        let x = unresolved_of069_with_6
            .iter()
            .find(|s| {
                let the1 = *resolved_numbers.get(&1).unwrap();
                (**s & the1).count_ones() == 1
            })
            .unwrap();
        resolved_numbers.insert(6, *x);
        // 9 innefattar hel 4:a
        let x = unresolved_of069_with_6
            .iter()
            .find(|s| {
                let the4 = *resolved_numbers.get(&4).unwrap();
                **s & the4 == the4
            })
            .unwrap();
        resolved_numbers.insert(9, *x);
        // 0
        let x = unresolved_of069_with_6
            .iter()
            .find(|s| {
                **s != *resolved_numbers.get(&6).unwrap()
                    && **s != *resolved_numbers.get(&9).unwrap()
            })
            .unwrap();
        resolved_numbers.insert(0, *x);

        let display: Vec<u8> = segs[1]
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| disp_charencoded_to_bitncoded(s))
            .collect();

        let decoded_ints: Vec<u8> = display
            .iter()
            .map(|s| {
                let x = resolved_numbers
                    .iter()
                    .find_map(|(key, &val)| if val == *s { Some(key) } else { None });
                *x.unwrap()
            })
            .collect();

        result += decoded_ints[0] as i32 * 1000;
        result += decoded_ints[1] as i32 * 100;
        result += decoded_ints[2] as i32 * 10;
        result += decoded_ints[3] as i32;
    });

    Ok(result)
}

//  a sets the 1 first bit b the second bit and so on. Only a-f are valid
fn disp_charencoded_to_bitncoded(s: &str) -> u8 {
    let mut i: u8 = 0;
    for ch in s.chars() {
        // let f = (ch as usize - 'a' as usize) as u8;
        let mut val = 1;
        val <<= ch as usize - 'a' as usize;
        i += val;
    }
    i
}

//  a sets the 1 first bit b the second bit and so on. Only a-f are valid
fn fmt_u8_seg(n: u8) -> String {
    let mut s = String::new();
    s.push_str(&format!("{}={:b} ", n, n));
    for i in 0..7 {
        let mut val = 1;
        val <<= i;
        if n & val > 0 {
            let c = ('a' as u8 + i) as char;
            s.push_str(&c.to_string());
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!(r"../../inputs/day08-example.txt");

    static INPUT_EXAMPLE_LAB: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn p1_solution() {
        assert_eq!(puzzle_1(INPUT).unwrap(), 421);
    }
    #[test]
    fn p2_solution() {
        assert_eq!(puzzle_2(INPUT).unwrap(), 986163);
    }
    #[test]
    fn p1_ex() {
        assert_eq!(puzzle_1(INPUT_EXAMPLE).unwrap(), 26);
    }

    #[test]
    fn p2_ex() {
        assert_eq!(puzzle_2(INPUT_EXAMPLE).unwrap(), 61229);
    }

    #[test]
    fn p2_dev() {
        assert_eq!(puzzle_2(INPUT_EXAMPLE_LAB).unwrap(), 5353);
    }
}
