fn get_spring_schema(arrangement_line: &str) -> String {
    arrangement_line.split(" ").collect::<Vec<&str>>()[0].to_string()
}

fn get_spring_numbers(arrangement_line: &str) -> Vec<u64> {
    let numbers_section = arrangement_line.split(" ").collect::<Vec<&str>>()[1];

    let mut numbers = Vec::new();

    numbers_section
        .split(",")
        .for_each(|number| {
            if number.parse::<u64>().is_ok() {
                numbers.push(number.parse::<u64>().unwrap());
            }
        });

    numbers
}

fn calculate_line_arrangements(spring_schema: String, spring_numbers: Vec<u64>) -> u64 {
    if spring_schema.find("?").is_none() {

        let split_schema = spring_schema.split(".").filter(|&section| !section.is_empty()).collect::<Vec<&str>>();

        if split_schema.len() != spring_numbers.len() {
            return 0;
        }

        for number_index in 0..spring_numbers.len() {
            if split_schema[number_index].len() != spring_numbers[number_index] as usize {
                return 0;
            }
        }

        return 1;
    }

    calculate_line_arrangements(spring_schema.replacen("?", ".", 1), spring_numbers.clone()) + calculate_line_arrangements(spring_schema.replacen("?", "#", 1), spring_numbers)
}

pub fn calculate_multiple_arrangements(arrangements: String) -> u64 {

    let mut result = 0;

    let arrangement_lines = arrangements.split("\r\n").collect::<Vec<&str>>();
    arrangement_lines.iter().for_each(|&arrangement_line| {
        let spring_schema = get_spring_schema(arrangement_line);
        let spring_numbers = get_spring_numbers(arrangement_line);

        let line_arrangements = calculate_line_arrangements(spring_schema, spring_numbers);
        result += line_arrangements;
    });

    result
}