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

    println!("Result: {}", get_max_output_joltage(buffer));

    Ok(())
}

pub fn get_largest_batteries_in_bank(bank: String) -> (i32, i32) {
    let bank_chars: Vec<char> = bank.chars().collect();
    let mut max1 = bank_chars[0].to_digit(10).unwrap() as i32;
    let last_battery_value = bank_chars[bank_chars.len() - 1].to_digit(10).unwrap() as i32;
    let mut max2 = last_battery_value;

    for battery in bank_chars[1..bank_chars.len() - 1].iter() {
        eprintln!("Battery: {} -> {max1} {max2}", battery);
        let battery_value = battery.to_digit(10).unwrap() as i32;

        if battery_value > max1 {
            max1 = battery_value;
            max2 = last_battery_value;
        } else if battery_value > max2 {
            max2 = battery_value;
        }
    }

    eprintln!("Max batteries in bank {}: {}, {}", bank, max1, max2);

    (max1, max2)
}

pub fn get_max_output_joltage(banks: String) -> i32 {
    let mut sum = 0;

    for bank in banks.lines() {
        let (a, b) = get_largest_batteries_in_bank(bank.to_string());
        sum += a * 10 + b;
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
        let result = get_max_output_joltage(range);
        assert_eq!(result, 357);
    }

    #[test]
    fn custom1() {
        let range = String::from("997654321111111");
        let result = get_max_output_joltage(range);
        assert_eq!(result, 99);
    }

    #[test]
    fn custom2() {
        let range = String::from("811811911111117");
        let result = get_max_output_joltage(range);
        assert_eq!(result, 97);
    }
}
