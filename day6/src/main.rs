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

    println!("Result: step 1:{}", get_grand_total(&buffer));
    println!("Result: step 2:{}", get_grand_total_2(&buffer));

    Ok(())
}

fn compute(numbers: &Vec<Vec<i32>>, operators: Vec<char>) -> i64 {
    let mut result = 0_i64;
    for i in 0..operators.len() {
        let mut sub_result = match operators[i] {
            '+' => 0_i64,
            '*' => 1_i64,
            _ => panic!("Unknown operator"),
        };

        for j in 0..numbers.len() {
            match operators[i] {
                '+' => sub_result += numbers[j][i] as i64,
                '*' => sub_result *= numbers[j][i] as i64,
                _ => panic!("Unknown operator"),
            }
        }

        eprintln!("Sub result for operator {}: {}", operators[i], sub_result);
        result += sub_result;
    }
    result
}

pub fn get_grand_total(buffer: &str) -> i64 {
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    let mut i = 0;

    for line in buffer.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        if parts[0].chars().all(|c| c.is_digit(10)) {
            numbers.push(Vec::new());
            for nums in parts.iter() {
                numbers[i].push(nums.parse::<i32>().unwrap());
            }
        } else {
            for nums in parts.iter() {
                operators.push(nums.chars().next().unwrap());
            }
        }
        i += 1;
    }

    eprintln!("Numbers: {:?}", numbers);
    eprintln!("Operators: {:?}", operators);

    compute(&numbers, operators.clone())
}

pub fn get_grand_total_2(buffer: &str) -> i64 {
    let mut numbers: Vec<i32> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for line in buffer.lines() {
        if !line.starts_with(&['*', '+']) {
            let mut i = 0;
            for digit in line.chars().rev() {
                if numbers.len() <= i {
                    numbers.push(0_i32);
                }

                if digit != ' ' {
                    numbers[i] *= 10;
                    numbers[i] += digit.to_digit(10).unwrap() as i32;
                }
                i += 1;
            }
        } else {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            for nums in parts.iter().rev() {
                operators.push(nums.chars().next().unwrap());
            }
        }
    }

    eprintln!("Numbers: {:?}", numbers);
    eprintln!("Operators: {:?}", operators);

    let mut j = 0;
    let mut result = 0_i64;
    for op in operators {
        result += match op {
            '+' => {
                let mut sub_result = 0_i64;
                while j < numbers.len() && numbers[j] != 0 {
                    sub_result += numbers[j] as i64;
                    j += 1;
                }
                sub_result
            }
            '*' => {
                let mut sub_result = 1_i64;
                while j < numbers.len() && numbers[j] != 0 {
                    sub_result *= numbers[j] as i64;
                    j += 1;
                }
                sub_result
            }
            _ => panic!("Unknown operator"),
        };

        j += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = String::from(
            "123 328  51 64 \n\
            45 64  387 23 \n\
            6 98  215 314\n\
            *   +   *   + \n",
        );

        let step1 = get_grand_total(&input);
        assert_eq!(step1, 4277556);
    }

    #[test]
    fn aoc_example_step2() {
        let input = String::from(
            "123 328  51 64 \n\
            45 64  387 23 \n\
            6 98  215 314\n\
            *   +   *   + \n",
        );

        let step1 = get_grand_total_2(&input);
        assert_eq!(step1, 3263827);
    }
}
