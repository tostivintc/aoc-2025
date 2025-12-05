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

    println!("Result step 1: {}", count_fresh_ingredients(&buffer));

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

pub fn count_fresh_ingredients(list: &str) -> usize {
    let mut count = 0_usize;
    let mut range_done = false;
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for line in list.lines() {
        if !range_done {
            match get_fresh_ranges_from_line(line) {
                Some(range) => ranges.push(range),
                None => range_done = true,
            }
        } else if is_ingredient_fresh(line, &ranges) {
            eprintln!("Fresh ingredient found: {}", line);
            count += 1;
        }
    }

    count
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

        let result = count_fresh_ingredients(&input);
        assert_eq!(result, 3);
    }
}
