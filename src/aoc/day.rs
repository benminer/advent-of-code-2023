use std::fs;

pub struct AdventOfCodeDay {
    pub day: u8,
}

impl AdventOfCodeDay {
    pub fn new(day: u8) -> AdventOfCodeDay {
        AdventOfCodeDay { day }
    }

    pub fn get_input<T>(&self, part: Option<u8>) -> Vec<String> {
        let part = match part {
            Some(part) => part,
            None => 1,
        };
        let file_path = format!("aoc/day{:02}/part{:02}.txt", self.day, part);
        println!("Reading file: {}", file_path);
        match fs::read_to_string(file_path) {
            Ok(content) => {
                return content
                    .split('\n')
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>();
            }
            Err(e) => {
                println!("Error reading file: {:?}", e);
                return Vec::new();
            }
        }
    }
}
