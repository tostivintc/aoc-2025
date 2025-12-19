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

    let (step1, step2) = get_tachyon_beam_split(&buffer);
    println!("Result: step 1:{step1}");
    println!("Result: step 1:{step2}");

    Ok(())
}

fn update_beam_positions(line: &str, beam_pos: &mut Vec<u64>) -> u32 {
    if beam_pos.is_empty() {
        for _i in 0..line.len() {
            beam_pos.push(0);
        }
    }

    let beam_pos_initial = beam_pos.clone();
    let mut split_count = 0_u32;

    for (i, c) in line.chars().enumerate() {
        if c == 'S' {
            beam_pos[i] += 1;
        } else if c == '^' && beam_pos_initial[i] > 0 {
            eprintln!("Beam split at position {}", i);
            split_count += 1;
            if i > 0 {
                beam_pos[i - 1] += beam_pos_initial[i];
            }
            beam_pos[i] = 0;
            if i + 1 < beam_pos.len() {
                beam_pos[i + 1] += beam_pos_initial[i];
            }
        }
    }
    eprintln!("Beam positions: {:?}", beam_pos);
    split_count
}

pub fn get_tachyon_beam_split(buffer: &str) -> (u32, u64) {
    let mut beam_pos: Vec<u64> = Vec::new();
    let mut splits = 0_u32;
    let mut timelines = 0_u64;

    for line in buffer.lines() {
        splits += update_beam_positions(line, &mut beam_pos);
        eprintln!("After line: '{}', total splits: {}", line, splits);
        eprintln!("line length: {}", line.len());
    }
    for point in &beam_pos {
        timelines += point;
    }
    (splits, timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = String::from(
            ".......S.......
            ...............\n\
            .......^.......\n\
            ...............\n\
            ......^.^......\n\
            ...............\n\
            .....^.^.^.....\n\
            ...............\n\
            ....^.^...^....\n\
            ...............\n\
            ...^.^...^.^...\n\
            ...............\n\
            ..^...^.....^..\n\
            ...............\n\
            .^.^.^.^.^...^.\n\
            ...............",
        );

        let (step1, step2) = get_tachyon_beam_split(&input);
        assert_eq!(step1, 21);
        assert_eq!(step2, 40);
    }
}
