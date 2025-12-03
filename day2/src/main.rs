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

    match f.read_to_string(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", file_path, why),
        Ok(_) => print!("{} contains:\n{}", file_path, buffer),
    }

    println!("Result: {}", invalid_id_sum(buffer));

    Ok(())
}

pub fn invalid_ids(start: usize, stop: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    eprintln!("Checking IDs from {} to {}", start, stop);

    for id in start..=stop {
        let id_str: String = id.to_string();
        let mid = id_str.len() / 2;

        if id_str[0..mid] == id_str[mid..] {
            result.push(id);
        }
    }

    eprintln!("Found {:?} invalid IDs", result);

    result
}

pub fn invalid_id_sum(ranges: String) -> u64 {
    let mut sum = 0_u64;

    for range in ranges.trim().split(',') {
        let bounds: Vec<&str> = range.split('-').collect();
        let start: usize = bounds[0].parse().unwrap();
        let stop: usize = bounds[1].parse().unwrap();

        let invalids = invalid_ids(start, stop);
        for id in invalids {
            sum += id as u64;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_ids() {
        let result = invalid_ids(80, 1011);
        assert_eq!(result, vec![88, 99, 1010]);
    }

    #[test]
    fn aoc_example() {
        let range = String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        let result = invalid_id_sum(range);
        assert_eq!(result, 1227775554);
    }
}
