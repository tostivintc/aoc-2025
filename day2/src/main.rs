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

    println!("Result step 1: {}", invalid_id_sum(&buffer, 2));
    println!("Result:step 2: {}", invalid_id_sum(&buffer, 0));

    Ok(())
}

fn split_n_times(id_str: &str, n: usize) -> Vec<&str> {
    let len = id_str.len();
    let part_size = len / n;
    let mut parts: Vec<&str> = Vec::new();

    for i in 0..n {
        let start = i * part_size;
        let end = if i == n - 1 { len } else { start + part_size };
        parts.push(&id_str[start..end]);
    }

    parts
}

pub fn invalid_ids(start: usize, stop: usize, max_times: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    eprintln!("Checking IDs from {} to {}", start, stop);

    for id in start..=stop {
        let id_str: String = id.to_string();

        let max = match max_times {
            0 => id_str.len(),
            _ => max_times,
        };

        let mut time = 2;
        while time <= max {
            let parts = split_n_times(&id_str, time);

            //eprintln!("ID: {}, parts: {:?}, result: {:?}", id, parts, result);

            if result.contains(&id) {
                break;
            }

            let mut match_found = true;
            for part in parts[1..].iter() {
                if *part != parts[0] {
                    match_found = false;
                    break;
                }
            }
            if match_found {
                result.push(id);
            }
            time += 1;
        }
    }

    eprintln!("Found {:?} invalid IDs", result);

    result
}

pub fn invalid_id_sum(ranges: &str, max_times: usize) -> u64 {
    let mut sum = 0_u64;

    for range in ranges.trim().split(',') {
        let bounds: Vec<&str> = range.split('-').collect();
        let start: usize = bounds[0].parse().unwrap();
        let stop: usize = bounds[1].parse().unwrap();

        let invalids = invalid_ids(start, stop, max_times);
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
    fn test_invalid_ids_half() {
        let result = invalid_ids(80, 1011, 2);
        assert_eq!(result, vec![88, 99, 1010]);
    }

    #[test]
    fn test_invalid_ids() {
        let result = invalid_ids(80, 111, 0);
        assert_eq!(result, vec![88, 99, 111]);
    }

    #[test]
    fn aoc_example_step1() {
        let range = String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        let result = invalid_id_sum(&range, 2);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn aoc_example_step2() {
        let range = String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        let result = invalid_id_sum(&range, 0);
        assert_eq!(result, 4174379265);
    }
}
