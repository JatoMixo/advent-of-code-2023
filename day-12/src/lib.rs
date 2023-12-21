fn get_spring_schema(arrangement_line: &str) -> String {
    arrangement_line.split(" ").collect::<Vec<&str>>()[0].to_string()
}

fn get_spring_numbers(arrangement_line: &str) -> Vec<u64> {
    let numbers_section = arrangement_line.split(" ").collect::<Vec<&str>>()[1];

    let mut numbers = Vec::new();

    numbers_section
        .split(",")
        .for_each(|number| {
            numbers.push(number.parse::<u64>().unwrap());
        });

    numbers
}

fn calculate_line_arrangements(spring_schema: String, spring_numbers: Vec<u64>) -> u64 {
    0
}

pub fn calculate_multiple_arrangements(arrangements: String) -> u64 {

    let mut result = 0;

    let arrangement_lines = arrangements.split("\n").collect::<Vec<&str>>();
    arrangement_lines.iter().for_each(|&arrangement_line| {
        let spring_schema = get_spring_schema(arrangement_line);
        let spring_numbers = get_spring_numbers(arrangement_line);

        let line_arrangements = calculate_line_arrangements(spring_schema, spring_numbers);
        result += line_arrangements;
    });

    result
}