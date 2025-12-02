use std::env;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut value = 50;
    let mut n_zero = 0;

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?; // handle or propagate the error
        let side = line.chars().nth(0).unwrap();
        let mut click = line[1..].parse::<i32>().unwrap();

        eprintln!("\n{}: {}", side, click);

        if side == 'L' {
            click *= -1;
        }

        let old_value = value;
        value += click;

        n_zero += click.abs() / 100;

        while value < 0 || value > 99 {
            if value < 0 {
                value += 100;
            } else if value > 99 {
                value -= 100;
            }
        }

        if side == 'L' && old_value < value && old_value != 0 {
            n_zero += 1;
        } else if side == 'L' && value == 0 {
            n_zero += 1;
        } else if side == 'R' && old_value > value {
            n_zero += 1;
        }

        eprintln!("value: {value}, click:{n_zero}");
    }

    println!("\nPassword: {n_zero}");

    Ok(())
}
