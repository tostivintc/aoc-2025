use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Device {
    name: String,
    link: Vec<usize>,
    is_out: bool,
    is_fft: bool,
    is_dac: bool,
    tree_done: bool,
    path_with_both: u64,
    path_with_fft: u64,
    path_with_dac: u64,
    path: u64,
}

pub struct Server {
    you: usize,
    svr: usize,
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
            svr: 0,
            devices: Vec::new(),
        };
        server.devices.push(Device {
            name: "out".to_string(),
            link: Vec::new(),
            is_out: true,
            is_fft: false,
            is_dac: false,
            tree_done: true,
            path_with_both: 0,
            path_with_fft: 0,
            path_with_dac: 0,
            path: 1,
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
            is_fft: false,
            is_dac: false,
            tree_done: false,
            path_with_both: 0,
            path_with_fft: 0,
            path_with_dac: 0,
            path: 0,
        };

        match name {
            "you" => self.you = self.devices.len(),
            "svr" => self.svr = self.devices.len(),
            "fft" => device.is_fft = true,
            "dac" => device.is_dac = true,
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
        eprintln!("Server dump: you:{} svr:{} ", self.you, self.svr);
        for (i, device) in self.devices.iter().enumerate() {
            eprintln!(
                "Device {}: is_out:{} is_dac:{}, is_fft:{}, name:{} links:{:?}",
                i, device.is_out, device.is_dac, device.is_fft, device.name, device.link
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

    /************************************************/

    fn count_paths_to_out_bis(&mut self, device_idx: usize) {
        if self.devices[device_idx].is_out {
            return;
        }

        let mut child_path_both = 0_u64;
        let mut child_path_fft = 0_u64;
        let mut child_path_dac = 0_u64;
        let mut child_path = 0_u64;
        for link_idx in 0..self.devices[device_idx].link.len() {
            let link = self.devices[device_idx].link[link_idx];

            if link != self.svr {
                if !self.devices[link].tree_done {
                    self.count_paths_to_out_bis(link);
                    self.devices[device_idx].tree_done = true;
                }

                child_path_both += self.devices[link].path_with_both;
                child_path_fft += self.devices[link].path_with_fft;
                child_path_dac += self.devices[link].path_with_dac;
                child_path += self.devices[link].path;
            } else {
                eprintln!("  Skipping link to svr to avoid loop");
            }
        }

        self.devices[device_idx].path_with_both = child_path_both;
        self.devices[device_idx].path = child_path;
        if self.devices[device_idx].is_fft {
            self.devices[device_idx].path_with_fft = child_path + child_path_fft;
            self.devices[device_idx].path_with_both += child_path_dac;
        } else if self.devices[device_idx].is_dac {
            self.devices[device_idx].path_with_dac = child_path + child_path_dac;
            self.devices[device_idx].path_with_both += child_path_fft;
        } else {
            self.devices[device_idx].path_with_fft = child_path_fft;
            self.devices[device_idx].path_with_dac = child_path_dac;
        }
    }

    pub fn get_number_of_paths_from_srv_to_out(&mut self) -> u64 {
        self.count_paths_to_out_bis(self.svr);
        self.devices[self.svr].path_with_both
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
    println!(
        "Number of path from srv to out: {}",
        server.get_number_of_paths_from_srv_to_out()
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

    #[test]
    fn aoc_example_step_2() {
        let input = String::from(
            "svr: aaa bbb\n\
            aaa: fft\n\
            fft: ccc\n\
            bbb: tty\n\
            tty: ccc\n\
            ccc: ddd eee\n\
            ddd: hub\n\
            hub: fff\n\
            eee: dac\n\
            dac: fff\n\
            fff: ggg hhh\n\
            ggg: out\n\
            hhh: out",
        );

        let mut server = Server::new().from_data(&input);
        server.dump();
        assert_eq!(2, server.get_number_of_paths_from_srv_to_out());
    }
}
