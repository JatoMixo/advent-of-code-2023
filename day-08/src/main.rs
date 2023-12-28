use std::collections::HashMap;

#[derive(Debug)]
struct Map {
    pub left: String,
    pub right: String,
}

impl Map {
    pub fn from_string(mut string: String) -> Map {
        string.remove(0);
        string.remove(string.len() - 1);

        let (left, right) = string.split_once(", ").unwrap();
        Map {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

fn get_maps(string: String) -> HashMap<String, Map> {
    let lines = string.split("\r\n").collect::<Vec<&str>>();

    let mut maps = HashMap::new();
    lines   
        .iter()
        .for_each(|line| {
            let (original, destination) = line.split_once(" = ").unwrap();


            maps.insert(original.to_string(), Map::from_string(destination.to_string()));
        });

    maps
}

fn calculate_steps(input: String) -> u64 {
    let (instructions, maps) = input.split_once("\r\n\r\n").unwrap();

    let mut location = maps.clone().split_once(" = ").unwrap().0.to_string();
    let mut steps: u64 = 0;

    let instructions = instructions.to_string();
    let maps = get_maps(maps.to_string());

    loop {
        let actual_instruction = instructions.chars().nth(steps as usize % instructions.len()).unwrap();

        if location == "ZZZ".to_string() {
            break;
        }
        
        steps += 1;
        if actual_instruction == 'L' {
            location = maps.get(&location).unwrap().left.clone();
            continue;
        }

        location = maps.get(&location).unwrap().right.clone();
    }

    steps
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("File");

    println!("{}", calculate_steps(input));
}
