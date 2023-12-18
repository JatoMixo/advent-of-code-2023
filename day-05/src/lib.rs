#[derive(Debug)]
struct Rule {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64
}

impl Rule {
    pub fn from_string(rule: String) -> Rule {
        let string_split = rule.split(" ").collect::<Vec<&str>>();

        Rule {
            destination_range_start: string_split[0].parse::<u64>().unwrap(),
            source_range_start: string_split[1].parse::<u64>().unwrap(),
            range_length: string_split[2].parse::<u64>().unwrap(),
        }
    }

    pub fn get(&self, source_number: u64) -> u64 {
        if source_number >= self.source_range_start && source_number < self.source_range_start + self.range_length {
            return (source_number - self.source_range_start) + self.destination_range_start;
        }

        source_number
    }
}

#[derive(Debug)]
struct Map {
    pub rules: Vec<Rule>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            rules: Vec::new()
        }
    }

    pub fn get(&self, data: u64) -> u64 {
        for rule in self.rules.iter() {
            let equivalent = rule.get(data);
            if equivalent != data {
                return equivalent;
            }
        };
    
        data
    }
}

fn get_lowest_number(data: Vec<u64>) -> u64 {
    *data.iter().min().unwrap()
}

fn get_seeds(data_lines: &Vec<&str>) -> Vec<u64> {

    let mut result = Vec::new();

    data_lines[0].replace("seeds: ", "").split(" ").for_each(|number| {
        result.push(number.trim().parse().unwrap());
    });

    result
}

fn is_map_identifier(line: &str) -> bool {
    line.contains(":")
}

pub fn get_lowest_location(data: String) -> u64 {
    let data_lines = data.split("\r\n").collect::<Vec<&str>>();

    let mut seeds = get_seeds(&data_lines);
    let mut actual_map = Map::new();
    for line_index in 0..data_lines.len() {
        let line = data_lines[line_index];

        if is_map_identifier(line) {
            continue;
        }

        if line.is_empty() || line_index == data_lines.len() - 1 {
            seeds.iter_mut().for_each(|seed| {
                *seed = actual_map.get(*seed);
            });

            actual_map = Map::new();
            continue;
        }

        actual_map.rules.push(Rule::from_string(line.to_string()));
    }

    get_lowest_number(seeds)
}

fn get_seed_with_ranges(data: &Vec<&str>) -> Vec<u64> {
    let numbers = get_seeds(data);


}

pub fn get_lowest_location_ranges(data: String) -> u64 {
    let data_lines = data.split("\r\n").collect::<Vec<&str>>();

    let mut seeds = get_seed_with_ranges(&data_lines);
    let mut actual_map = Map::new();
    for line_index in 0..data_lines.len() {
        let line = data_lines[line_index];

        if is_map_identifier(line) {
            continue;
        }

        if line.is_empty() || line_index == data_lines.len() - 1 {
            seeds.iter_mut().for_each(|seed| {
                *seed = actual_map.get(*seed);
            });

            actual_map = Map::new();
            continue;
        }

        actual_map.rules.push(Rule::from_string(line.to_string()));
    }

    get_lowest_number(seeds)
}