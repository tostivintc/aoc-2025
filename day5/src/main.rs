use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut f = match File::open(file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

    let mut buffer = String::new();
    if let Err(why) = f.read_to_string(&mut buffer) {
        panic!("couldn't read {}: {}", file_path, why)
    }

    let (step1, step2) = count_fresh_ingredients(&buffer);
    println!("Result: step 1:{step1}, step 2:{step2}",);

    Ok(())
}

fn get_fresh_ranges_from_line(buffer: &str) -> Option<(usize, usize)> {
    if buffer.contains('-') {
        let value = buffer.split('-').collect::<Vec<&str>>();

        eprintln!("Value parts: {:?}", value);

        if value.len() == 2 {
            let start = value[0].parse::<usize>().unwrap();
            let stop = value[1].parse::<usize>().unwrap();

            return Some((start, stop));
        }
    }

    None
}

fn is_ingredient_fresh(ingredient: &str, ranges: &Vec<(usize, usize)>) -> bool {
    let ingredient_value = ingredient.parse::<usize>().unwrap();

    for range in ranges {
        if ingredient_value >= range.0 && ingredient_value <= range.1 {
            return true;
        }
    }

    false
}

fn optimize_ranges(ranges: &mut Vec<(usize, usize)>) {
    ranges.sort();

    let mut i = 0;
    while i < ranges.len() - 1 {
        let stop = ranges[i].1;
        let start_next = ranges[i + 1].0;

        if stop >= start_next - 1 {
            if stop < ranges[i + 1].1 {
                ranges[i].1 = ranges[i + 1].1;
            }
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

pub fn count_fresh_ingredients(list: &str) -> (usize, usize) {
    let mut count = 0_usize;
    let mut range_done = false;
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for line in list.lines() {
        if !range_done {
            match get_fresh_ranges_from_line(line) {
                Some(range) => ranges.push(range),
                None => {
                    range_done = true;
                    optimize_ranges(&mut ranges);
                }
            }
        } else if is_ingredient_fresh(line, &ranges) {
            eprintln!("Fresh ingredient found: {}", line);
            count += 1;
        }
    }

    // Count number a fresh ingredient IDs
    let mut fresh_id = 0_usize;
    eprintln!("Fresh ranges: {:#?}", ranges);
    for range in ranges {
        fresh_id += range.1 - range.0 + 1;
    }

    (count, fresh_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = String::from(
            "3-5\n\
            10-14\n\
            16-20\n\
            12-18\n\
            \n\
            1\n\
            5\n\
            8\n\
            11\n\
            17\n\
            32",
        );

        let (n_fresh, n_fresh_id) = count_fresh_ingredients(&input);
        assert_eq!(n_fresh, 3);
        assert_eq!(n_fresh_id, 14);
    }

    #[test]
    fn extended() {
        let input = String::from(
            "3-5\n\
            10-20\n\
            16-18\n\
            \n\
            1\n\
            5\n\
            8\n\
            11\n\
            17\n\
            32",
        );

        let (n_fresh, n_fresh_id) = count_fresh_ingredients(&input);
        assert_eq!(n_fresh, 3);
        assert_eq!(n_fresh_id, 14);
    }
}
