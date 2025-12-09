use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Box {
    x: i64,
    y: i64,
    z: i64,
}

struct Distance {
    distance: f64,
    a: usize,
    b: usize,
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

    println!(
        "Result: step 1:{}",
        get_size_of_largest_circuit(&buffer, 3, 1000)
    );
    println!("Result: step 2:{}", get_last_link(&buffer));

    Ok(())
}

fn get_distance(box1: &Box, box2: &Box) -> f64 {
    let dx = (box1.x - box2.x) as f64;
    let dy = (box1.y - box2.y) as f64;
    let dz = (box1.z - box2.z) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn get_boxes_distances(boxes: &[Box]) -> Vec<Distance> {
    let mut distances: Vec<Distance> = Vec::new();

    for i in 0..boxes.len() {
        let j_start = i + 1;
        for j in j_start..boxes.len() {
            distances.push(Distance {
                distance: get_distance(&boxes[i], &boxes[j]),
                a: i,
                b: j,
            });
        }
    }

    distances.sort_by_key(|d| d.distance as i64);

    for d in distances.iter() {
        eprintln!(
            "Distance between box {} and box {}: {}",
            d.a, d.b, d.distance
        );
    }
    distances
}

fn add_link(circuit: &mut Vec<Vec<usize>>, d: &Distance) {
    let mut a_circuit = None;
    let mut b_circuit = None;

    for (c_i, c) in circuit.iter().enumerate() {
        if c.contains(&d.a) {
            a_circuit = Some(c_i);
        }
        if c.contains(&d.b) {
            b_circuit = Some(c_i);
        }
        if a_circuit.is_some() && b_circuit.is_some() {
            break;
        }
    }

    if a_circuit.is_none() && b_circuit.is_none() {
        circuit.push(vec![d.a, d.b]);
    } else if a_circuit.is_some() && b_circuit.is_some() {
        let ac = a_circuit.unwrap();
        let bc = b_circuit.unwrap();

        if ac != bc {
            let mut to_append = circuit[bc].clone();
            circuit[ac].append(&mut to_append);
            circuit.remove(bc);
        } else {
            eprintln!(
                "Linked ignoed as boxes {} and {} are already in the same circuit {}",
                d.a, d.b, ac
            );
        }
    } else if let Some(ac) = a_circuit {
        circuit[ac].push(d.b);
    } else {
        let bc = b_circuit.unwrap();
        circuit[bc].push(d.a);
    }
    eprintln!("Current circuit: {:?}", circuit);
}

fn get_circuit(distance: Vec<Distance>, max: usize) -> Vec<Vec<usize>> {
    let mut circuit: Vec<Vec<usize>> = Vec::new();

    for d in distance.iter().take(max) {
        add_link(&mut circuit, d);
    }

    circuit
}

fn get_single_circuit(distance: Vec<Distance>, max: usize) -> (usize, usize) {
    let mut circuit: Vec<Vec<usize>> = Vec::new();

    for d in distance.iter() {
        add_link(&mut circuit, d);
        if circuit.len() == 1 && circuit[0].len() == max {
            eprintln!("Single circuit found!");
            return (d.a, d.b);
        }
    }

    panic!("No single circuit found");
}

fn get_boxes_from_string(buffer: &str) -> Vec<Box> {
    let mut boxes: Vec<Box> = Vec::new();

    for line in buffer.lines() {
        let parts = line.split(',').collect::<Vec<&str>>();
        if parts.len() != 3 {
            panic!("Invalid input line: {}", line);
        }
        boxes.push(Box {
            x: parts[0].parse::<i64>().unwrap(),
            y: parts[1].parse::<i64>().unwrap(),
            z: parts[2].parse::<i64>().unwrap(),
        });
    }

    boxes
}

pub fn get_size_of_largest_circuit(buffer: &str, count: usize, n_link: usize) -> usize {
    let boxes = get_boxes_from_string(buffer);

    let distances = get_boxes_distances(&boxes);

    let mut circuit = get_circuit(distances, n_link);
    circuit.sort_by_key(|b| std::cmp::Reverse(b.len()));
    eprintln!("Final circuit:{:#?}", circuit);

    let mut result = 1;
    for c in circuit.iter().take(count) {
        result *= c.len();
    }

    result
}

pub fn get_last_link(buffer: &str) -> i64 {
    let boxes = get_boxes_from_string(buffer);

    let distances = get_boxes_distances(&boxes);

    let (a, b) = get_single_circuit(distances, boxes.len());

    boxes[a].x * boxes[b].x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = String::from(
            "162,817,812\n\
            57,618,57\n\
            906,360,560\n\
            592,479,940\n\
            352,342,300\n\
            466,668,158\n\
            542,29,236\n\
            431,825,988\n\
            739,650,466\n\
            52,470,668\n\
            216,146,977\n\
            819,987,18\n\
            117,168,530\n\
            805,96,715\n\
            346,949,466\n\
            970,615,88\n\
            941,993,340\n\
            862,61,35\n\
            984,92,344\n\
            425,690,689",
        );

        let step1 = get_size_of_largest_circuit(&input, 3, 10);
        assert_eq!(step1, 40);
    }

    #[test]
    fn aoc_example_step2() {
        let input = String::from(
            "162,817,812\n\
            57,618,57\n\
            906,360,560\n\
            592,479,940\n\
            352,342,300\n\
            466,668,158\n\
            542,29,236\n\
            431,825,988\n\
            739,650,466\n\
            52,470,668\n\
            216,146,977\n\
            819,987,18\n\
            117,168,530\n\
            805,96,715\n\
            346,949,466\n\
            970,615,88\n\
            941,993,340\n\
            862,61,35\n\
            984,92,344\n\
            425,690,689",
        );

        let step2 = get_last_link(&input);
        assert_eq!(step2, 25272);
    }
}
