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

    println!("Result step 1: {}", get_accessible_rolls_number(&buffer));

    println!(
        "Result step 2: {}",
        get_accessible_rolls_number_loop(&buffer)
    );

    Ok(())
}

pub fn get_accessible_rolls_number_loop(map: &str) -> usize {
    let mut result = 0_usize;
    let lines: Vec<&str> = map.lines().collect();
    let mut map_chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    loop {
        let round_result = get_accessible_rolls_number_vec(&mut map_chars);
        if round_result == 0 {
            break;
        }
        eprintln!("Round result: {}", round_result);
        result += round_result;
    }

    result
}

pub fn get_accessible_rolls_number(map: &str) -> usize {
    let lines: Vec<&str> = map.lines().collect();
    let mut map_chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    eprintln!("Chars map: {map_chars:?}");

    get_accessible_rolls_number_vec(&mut map_chars)
}

pub fn get_accessible_rolls_number_vec(map: &mut Vec<Vec<char>>) -> usize {
    let mut accessible_count = 0;
    let height = map.len();
    let map_orginal = map.to_owned();

    for y in 0..map_orginal.len() {
        let width = map_orginal[y].len();

        for x in 0..width {
            if map_orginal[y][x] == '@' {
                let mut n_adjacent = 0;

                // Check 6 up
                if y > 0 {
                    if x > 0 && map_orginal[y - 1][x - 1] == '@' {
                        n_adjacent += 1;
                    }
                    if map_orginal[y - 1][x] == '@' {
                        n_adjacent += 1;
                    }
                    if x < width - 1 && map_orginal[y - 1][x + 1] == '@' {
                        n_adjacent += 1;
                    }
                }

                // Check left
                if x > 0 && map_orginal[y][x - 1] == '@' {
                    n_adjacent += 1;
                }

                // Check right
                if x < width - 1 && map_orginal[y][x + 1] == '@' {
                    n_adjacent += 1;
                }

                // Check 6 down
                if y < height - 1 {
                    if x > 0 && map_orginal[y + 1][x - 1] == '@' {
                        n_adjacent += 1;
                    }
                    if map_orginal[y + 1][x] == '@' {
                        n_adjacent += 1;
                    }
                    if x < width - 1 && map_orginal[y + 1][x + 1] == '@' {
                        n_adjacent += 1;
                    }
                }

                if n_adjacent < 4 {
                    accessible_count += 1;
                    map[y][x] = '.';
                }
            }
        }
    }

    accessible_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = String::from(
            "..@@.@@@@.\n\
            @@@.@.@.@@\n\
            @@@@@.@.@@\n\
            @.@@@@..@.\n\
            @@.@@@@.@@\n\
            .@@@@@@@.@\n\
            .@.@.@.@@@\n\
            @.@@@.@@@@\n\
            .@@@@@@@@.\n\
            @.@.@@@.@.",
        );

        let result = get_accessible_rolls_number(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn aoc_example_step2() {
        let input = String::from(
            "..@@.@@@@.\n\
            @@@.@.@.@@\n\
            @@@@@.@.@@\n\
            @.@@@@..@.\n\
            @@.@@@@.@@\n\
            .@@@@@@@.@\n\
            .@.@.@.@@@\n\
            @.@@@.@@@@\n\
            .@@@@@@@@.\n\
            @.@.@@@.@.",
        );

        let result = get_accessible_rolls_number_loop(&input);
        assert_eq!(result, 43);
    }
}
