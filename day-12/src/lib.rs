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

fn get_sections(spring_schema: &String) -> Vec<&str> {
    // Get basic Vector of elements in between dots
    let spring_vector = spring_schema.split(".").collect::<Vec<&str>>();

    // Remove empty elements created by several dots one after the other (....)
    spring_vector
        .into_iter()
        .filter(|&section| !section.is_empty())
        .collect::<Vec<&str>>()
}

#[derive(Debug)]
struct SpringCombination {
    pub schema: String,
    pub numbers: Vec<u64>,
}

impl SpringCombination {
    pub fn new() -> SpringCombination {
        SpringCombination {
            schema: String::new(),
            numbers: Vec::new(),
        }
    }
}

fn get_spring_combinations(sections: &Vec<&str>, spring_numbers: &Vec<u64>) -> Vec<SpringCombination> {
    let mut result = Vec::new();
    let mut actual_number: usize = 0;

    sections.iter().for_each(|&section| {
        let mut numbers_sum = 0;
        let mut numbers_length = 0;

        let mut actual_combination = SpringCombination::new();
        actual_combination.schema = section.to_string();

        for number_index in actual_number..spring_numbers.len() {

            let number = spring_numbers[number_index]; 

            numbers_sum += number;
            numbers_length += 1;

            if section.len() as u64 >= numbers_sum + (numbers_length as f32/2f32).floor() as u64 {
                actual_combination.numbers.push(number);
                actual_number += 1;
            }
        };

        result.push(actual_combination);
    });

    result
}

fn get_combination_posibilities(spring_combination: &SpringCombination) -> u64 {
    0
}

fn calculate_line_arrangements(spring_schema: String, spring_numbers: Vec<u64>) -> u64 {
    let sections = get_sections(&spring_schema);
    let spring_combinations = get_spring_combinations(&sections, &spring_numbers);

    let mut result = 1;

    for combination_index in 0..spring_combinations.len() {
        result *= get_combination_posibilities(&spring_combinations[combination_index]);
    }

    result
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