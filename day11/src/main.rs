use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Device {
    name: String,
    link: Vec<usize>,
    is_out: bool,
}

pub struct Server {
    you: usize,
    devices: Vec<Device>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Server {
        let mut server = Server {
            you: 0,
            devices: Vec::new(),
        };
        server.devices.push(Device {
            name: "out".to_string(),
            link: Vec::new(),
            is_out: true,
        });

        server
    }

    pub fn from_data(mut self, data: &str) -> Server {
        for line in data.lines() {
            self.add_device_from_line(line);
        }
        for (i, line) in data.lines().enumerate() {
            self.set_device_link_from_line(i + 1, line);
        }
        self
    }

    fn add_device_from_line(&mut self, device_data: &str) {
        let data_split = device_data.split(":").collect::<Vec<&str>>();
        let name = data_split[0].trim();
        let mut device = Device {
            name: name.to_string(),
            link: Vec::new(),
            is_out: false,
        };

        match name {
            "you" => self.you = self.devices.len(),
            "out" => device.is_out = true,
            _ => {}
        }
        self.devices.push(device);
    }

    fn set_device_link_from_line(&mut self, device_idx: usize, device_data: &str) {
        let data_split = device_data.split(":").collect::<Vec<&str>>();
        let links_str = data_split[1].trim().split(' ').collect::<Vec<&str>>();

        for link in links_str {
            let link_index = self.find_device_index_by_name(link);
            self.devices[device_idx].link.push(link_index);
        }
    }

    fn find_device_index_by_name(&self, name: &str) -> usize {
        for (i, device) in self.devices.iter().enumerate() {
            if device.name == name {
                return i;
            }
        }
        panic!("Device with name {} not found", name);
    }

    pub fn dump(&mut self) {
        eprintln!("Server dump: you:{} ", self.you);
        for (i, device) in self.devices.iter().enumerate() {
            eprintln!(
                "Device {}: is_out:{} name:{} links:{:?}",
                i, device.is_out, device.name, device.link
            );
        }
    }

    fn count_paths_to_out(&mut self, device_idx: usize) -> u64 {
        if self.devices[device_idx].is_out {
            return 1;
        }

        let mut path_count = 0_u64;
        for link_idx in 0..self.devices[device_idx].link.len() {
            let link = self.devices[device_idx].link[link_idx];
            path_count += self.count_paths_to_out(link);
        }
        path_count
    }

    pub fn get_number_of_paths_to_out(&mut self) -> u64 {
        self.count_paths_to_out(self.you)
    }
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

    let mut server = Server::new().from_data(&buffer);
    server.dump();
    println!(
        "Number of path from you to out: {}",
        server.get_number_of_paths_to_out()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = String::from(
            "aaa: you hhh\n\
            you: bbb ccc\n\
            bbb: ddd eee\n\
            ccc: ddd eee fff\n\
            ddd: ggg\n\
            eee: out\n\
            fff: out\n\
            ggg: out\n\
            hhh: ccc fff iii\n\
            iii: out",
        );

        let mut server = Server::new().from_data(&input);
        server.dump();
        assert_eq!(5, server.get_number_of_paths_to_out());
    }
}
