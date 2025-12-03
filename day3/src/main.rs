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

    println!(
        "Result step 1: {}",
        get_max_output_joltage(buffer.clone(), 2)
    );
    println!(
        "Result step 2: {}",
        get_max_output_joltage(buffer.clone(), 12)
    );

    Ok(())
}

fn get_max(bank: &[i32], start: usize, stop: usize) -> (i32, usize) {
    let mut max = &bank[start];
    let mut max_index = start;

    for (i, value) in bank.iter().enumerate().take(stop + 1).skip(start) {
        if value > max {
            max = value;
            max_index = i;
        }
    }

    (*max, max_index)
}

pub fn get_largest_batteries_in_bank(bank: String, size: usize) -> Vec<i32> {
    let mut bank_int: Vec<i32> = Vec::new();
    let mut result: Vec<i32> = Vec::new();

    for value in bank.chars().collect::<Vec<_>>().iter() {
        bank_int.push(value.to_digit(10).unwrap() as i32);
    }

    let mut start = 0_usize;
    let mut stop = bank_int.len() - size;
    for _i in 0..size {
        let (max, j) = get_max(&bank_int, start, stop);
        result.push(max);
        start = j + 1;
        stop += 1;
    }

    eprintln!("Max batteries in bank {bank} {size}: {:?}", result);

    result
}

pub fn get_max_output_joltage(banks: String, size: usize) -> u64 {
    let mut sum = 0_u64;

    for bank in banks.lines() {
        let res = get_largest_batteries_in_bank(bank.to_string(), size);
        let mut bank_value = 0_u64;

        for value in res {
            bank_value *= 10;
            bank_value += value as u64;
        }
        sum += bank_value;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let range =
            String::from("987654321111111\n811111111111119\n234234234234278\n818181911112111\n");
        let result = get_max_output_joltage(range, 2);
        assert_eq!(result, 357);
    }

    #[test]
    fn aoc_example_step2() {
        let range =
            String::from("987654321111111\n811111111111119\n234234234234278\n818181911112111\n");
        let result = get_max_output_joltage(range, 12);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn custom1() {
        let range = String::from("997654321111111");
        let result = get_max_output_joltage(range, 2);
        assert_eq!(result, 99);
    }

    #[test]
    fn custom2() {
        let range = String::from("811811911111117");
        let result = get_max_output_joltage(range, 2);
        assert_eq!(result, 97);
    }
}
