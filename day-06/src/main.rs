#[derive(Debug)]
struct Race {
    pub times: Vec<u32>,
    pub records: Vec<u32>,
}

impl Race {
    pub fn from_string(string: String) -> Race {
        let rows = string.split("\n").collect::<Vec<&str>>();

        let times_section = rows[0].strip_prefix("Time: ").unwrap().split(" ").filter(|&number| number.parse::<u32>().is_ok()).collect::<Vec<&str>>();
        let mut times = Vec::new();
        times_section.iter().for_each(|number| {
            times.push(number.parse::<u32>().unwrap());
        });

        let records_section = rows[1].strip_prefix("Distance: ").unwrap().split(" ").filter(|&number| number.parse::<u32>().is_ok()).collect::<Vec<&str>>();
        let mut records = Vec::new();
        records_section.iter().for_each(|number| {
            records.push(number.parse::<u32>().unwrap());
        });

        Race {
            times: times,
            records: records,
        }
    }
}

fn calculate_distance(max_time: u32, time: u32) -> u32 {
    (max_time - time) * time
}

fn calculate_ways_for_race(race: Race) -> u32 {

    let mut result = 1;

    for record_index in 0..race.records.len() {
        let record = race.records[record_index];
        let time = race.times[record_index];

        let mut posibilities = 0;

        for actual_time in 1..time {
            if calculate_distance(time, actual_time as u32) >= record {
                println!("Explanation now");
                posibilities += 1;
            }
        }

        result *= posibilities;
    }

    result
}

fn main() {
    let race = Race::from_string("Time:        59     70     78     78
Distance:   430   1218   1213   1276".to_string());

    println!("Result: {}", calculate_ways_for_race(race));
}