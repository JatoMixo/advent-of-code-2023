use std::collections::HashMap;
use num_integer::lcm;

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

    let mut location = "AAA".to_string();
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

fn calculate_simultaneous_steps(input: String) -> u64 {
    let (instructions, maps) = input.split_once("\r\n\r\n").unwrap();

    let instructions = instructions.to_string();
    let maps = get_maps(maps.to_string());

    let mut locations = maps.keys().filter(|&map| map.ends_with("A")).collect::<Vec<&String>>();
    let mut steps_vec: Vec<u64> = Vec::new();
    let mut steps = 0;

    loop {
        let actual_instruction = instructions.chars().nth(steps as usize % instructions.len()).unwrap();

        if locations.iter().filter(|&&map| map.ends_with("Z")).collect::<Vec<&&String>>().len() == locations.len() {
            break;
        }
        
        steps += 1;
        for location_index in 0..locations.len() {

            let location = locations[location_index];

            if location.ends_with("Z") {
                continue;
            }

            match actual_instruction {
                'L' => {
                    let new_location = &maps.get(location).unwrap();
                    locations[location_index] = &new_location.left;
                },
                'R' => {
                    let new_location = &maps.get(location).unwrap();
                    locations[location_index] = &new_location.right;
                },
                _ => panic!("Wait, what just happened?!"),
            }

            if locations[location_index].ends_with("Z") {
                steps_vec.push(steps);
            }
        };
    }

    multiple_lcm(steps_vec)
}

fn multiple_lcm(vec: Vec<u64>) -> u64 {
    let mut result = lcm(vec[0], vec[1]);
    for element_index in 2..vec.len() {
        result = lcm(result, vec[element_index]);
    }
    result
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("File");

    println!("Result 1: {}", calculate_steps(input.clone()));
    println!("Result 2: {}", calculate_simultaneous_steps(input));
}
