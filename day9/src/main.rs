use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Pos {
    x: i64,
    y: i64,
}

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

    println!("Result: step 1:{}", get_largest_rectangle(&buffer));

    Ok(())
}

fn get_red_tile_from_string(buffer: &str) -> Vec<Pos> {
    let mut red_tiles: Vec<Pos> = Vec::new();

    for line in buffer.lines() {
        let parts = line.split(',').collect::<Vec<&str>>();
        let x = parts[0].parse::<i64>().unwrap();
        let y = parts[1].parse::<i64>().unwrap();
        red_tiles.push(Pos { x: x, y: y });
    }

    red_tiles
}

pub fn get_largest_rectangle(buffer: &str) -> i64 {
    let red_tiles = get_red_tile_from_string(buffer);
    eprintln!("Red tiles: {}", red_tiles.len());

    let mut max_area = 0;

    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let width = (red_tiles[i].x - red_tiles[j].x).abs() + 1;
            let height = (red_tiles[i].y - red_tiles[j].y).abs() + 1;
            let area = width * height;
            if area > max_area {
                eprintln!(
                    "New max area found: {} (from points ({},{}) and ({},{}))",
                    area, red_tiles[i].x, red_tiles[i].y, red_tiles[j].x, red_tiles[j].y
                );
                max_area = area;
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = String::from(
            "7,1\n\
            11,1\n\
            11,7\n\
            9,7\n\
            9,5\n\
            2,5\n\
            2,3\n\
            7,3",
        );

        let step1 = get_largest_rectangle(&input);
        assert_eq!(step1, 50);
    }
}
