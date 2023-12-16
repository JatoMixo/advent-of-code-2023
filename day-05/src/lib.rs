struct Rule {
    pub destination_range_start: u32,
    pub source_range_start: u32,
    pub range_length: u32
}

impl Rule {
    pub fn new() -> Rule {
        Rule {
            destination_range_start: 0,
            source_range_start: 0,
            range_length: 0,
        }
    }

    pub fn from_string(Rule: String) -> Rule {
        let string_split = Rule.split(" ").collect::<Vec<&str>>();

        Rule {
            destination_range_start: string_split[0].parse::<u32>().unwrap(),
            source_range_start: string_split[1].parse::<u32>().unwrap(),
            range_length: string_split[2].parse::<u32>().unwrap(),
        }
    }

    pub fn get(&self, source_number: u32) -> u32 {
        
        if (self.source_range_start..self.source_range_start + self.range_length).contains(&source_number) {
            return (source_number - self.source_range_start) + self.destination_range_start;
        }

        source_number
    }
}

pub fn get_lowest_location(data: String) -> u32 {

}