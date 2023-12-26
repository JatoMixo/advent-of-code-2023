#[derive(Debug)]
struct Race {
    pub times: Vec<u64>,
    pub records: Vec<u64>,
}

impl Race {
    pub fn from_string(string: String) -> Race {
        let rows = string.split("\n").collect::<Vec<&str>>();

        let times_section = rows[0].strip_prefix("Time: ").unwrap().split(" ").filter(|&number| number.parse::<u64>().is_ok()).collect::<Vec<&str>>();
        let mut times = Vec::new();
        times_section.iter().for_each(|number| {
            times.push(number.parse::<u64>().unwrap());
        });

        let records_section = rows[1].strip_prefix("Distance: ").unwrap().split(" ").filter(|&number| number.parse::<u64>().is_ok()).collect::<Vec<&str>>();
        let mut records = Vec::new();
        records_section.iter().for_each(|number| {
            records.push(number.parse::<u64>().unwrap());
        });

        Race {
            times: times,
            records: records,
        }
    }
}

fn calculate_distance(max_time: u64, time: u64) -> u64 {
    (max_time - time) * time
}

fn calculate_ways_for_race(race: Race) -> u64 {

    let mut result = 1;

    for record_index in 0..race.records.len() {
        let record = race.records[record_index];
        let time = race.times[record_index];

        let mut posibilities = 0;

        for actual_time in 1..time {
            if calculate_distance(time, actual_time as u64) >= record {
                posibilities += 1;
            }
        }

        result *= posibilities;
    }

    result
}

fn extract_values_string(string: String) -> (u64, u64) {
    let rows = string.split("\n").collect::<Vec<&str>>();
    
    let time = rows[0].strip_prefix("Time: ").unwrap().replace(" ", "").parse::<u64>().unwrap();
    let record = rows[1].strip_prefix("Distance: ").unwrap().replace(" ", "").parse::<u64>().unwrap();

    (time, record)
}

fn calculate_single_value(string: String) -> u64 {
    let (time, record) = extract_values_string(string);

    calculate_ways_for_race(Race {times: vec![time], records: vec![record]})
}

fn main() {
    let input = "Time:        59     70     78     78
Distance:   430   1218   1213   1276".to_string();

    // let race = Race::from_string(input.clone());

    // println!("Result 1: {}", calculate_ways_for_race(race));
    println!("Result 2: {}", calculate_single_value(input));
}