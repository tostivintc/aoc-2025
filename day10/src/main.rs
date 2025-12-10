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

    println!("Result: step 1:{}", get_total_num_button_to_push(&buffer));

    Ok(())
}

fn get_light_negative(buffer: &str) -> Vec<usize> {
    let mut light: Vec<usize> = Vec::new();
    for (i, c) in buffer.chars().enumerate() {
        if c == '#' {
            light.push(i - 1);
        }
    }

    light
}

fn get_button(parts: &[&str]) -> Vec<Vec<usize>> {
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    for part in parts.iter() {
        if part.starts_with('(') && part.ends_with(')') {
            let mut button: Vec<usize> = Vec::new();
            for btn in part[1..part.len() - 1]
                .split(',')
                .collect::<Vec<_>>()
                .iter()
            {
                button.push(btn.parse::<usize>().unwrap());
            }
            buttons.push(button);
        } else {
            panic!("Invalid button format");
        }
    }

    buttons
}

fn light_toggle(light: &Vec<usize>, button: &Vec<usize>) -> Vec<usize> {
    let mut new_light = light.clone();

    for b in button.iter() {
        let mut toggled = false;
        for i in 0..new_light.len() {
            if new_light[i] == *b {
                new_light.remove(i);
                toggled = true;
                break;
            } else if new_light[i] > *b {
                break;
            }
        }
        if toggled == false {
            new_light.push(*b);
            new_light.sort();
        }
    }

    new_light
}

fn check_press_next_button(
    light: &Vec<usize>,
    buttons: &Vec<Vec<usize>>,
    start_button: usize,
    num_pressed: i32,
) -> i32 {
    let mut min_buttons = -1;

    for b in start_button..buttons.len() {
        let tmp_light = light_toggle(&light, &buttons[b]);
        eprintln!(
            "Pressing button {} (start:  {start_button}) toggles lights to {:?}",
            b, tmp_light
        );
        if tmp_light.len() == 0 {
            eprintln!("All lights turned off with {} button(s)", num_pressed + 1);
            return num_pressed + 1;
        } else {
            let result = check_press_next_button(&tmp_light, buttons, b + 1, num_pressed + 1);
            if result != -1 {
                if min_buttons == -1 || result < min_buttons {
                    min_buttons = result;
                }
            }
        }
    }

    min_buttons
}

pub fn get_num_button_to_push(line: &str) -> i32 {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let light = get_light_negative(parts[0]);
    let button = get_button(parts[1..].split_last().unwrap().1);
    eprintln!("Light positions: {:?}", light);
    eprintln!("Buttons: {:?}", button);

    check_press_next_button(&light, &button, 0, 0)
}

pub fn get_total_num_button_to_push(buffer: &str) -> i32 {
    let mut total_buttons = 0;

    for line in buffer.lines() {
        total_buttons += get_num_button_to_push(line);
    }

    total_buttons
}

/*********************** Tests ***********************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_toggle_check() {
        let light = Vec::from([1, 3, 4]);
        let button = Vec::from([0, 2, 4, 5]);
        let result = light_toggle(&light, &button);
        assert_eq!(result, Vec::from([0, 1, 2, 3, 5]));
    }
    #[test]
    fn aoc_example() {
        let input = String::from(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );

        let step1 = get_total_num_button_to_push(&input);
        assert_eq!(step1, 7);
    }
}
