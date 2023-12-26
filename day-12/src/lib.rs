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

fn get_substring(string: &String, starting_index: usize, ending_index: usize) -> String {
    let mut result = String::new();

    for character_index in starting_index..ending_index {
        result += &string.chars().nth(character_index).unwrap().to_string();
    }

    result
}

fn get_vector_section(vector: &Vec<u64>, starting: usize, ending: usize) -> Vec<u64> {
    let mut result = Vec::new();

    for element_index in starting..ending {
        result.push(vector[element_index]);
    }

    result
}

fn get_combination_posibilities(spring_combination: &SpringCombination) -> u64 {

    let mut posibilities_per_number: Vec<u64> = Vec::new();
    for number_index in 0..spring_combination.numbers.len() {
        let number = spring_combination.numbers[number_index];

        let numbers_before = get_vector_section(&spring_combination.numbers, 0, number_index);
        let numbers_after = get_vector_section(&spring_combination.numbers, number_index + 1, spring_combination.numbers.len());

        let mut starting_position = numbers_before.iter().sum::<u64>() + numbers_before.len() as u64;
        if spring_combination.schema.find("#").unwrap_or(5) == 1 && starting_position != 0 {
            starting_position += 1;
        }
        let ending_position = spring_combination.schema.len() as u64 - (numbers_after.iter().sum::<u64>() + numbers_after.len() as u64);
        
        let number_section = get_substring(&spring_combination.schema, starting_position as usize, ending_position as usize);
        
        if number_section.find("#").is_none() {
            println!("{}", number_section);
            posibilities_per_number.push(number_section.len() as u64 - number + 1);
            continue;
        }
        
        let mut actual_probabilities = number_section.len() as u64 - number + 1;
        let section_split = number_section.split("#").filter(|&character| !character.is_empty()).collect::<Vec<&str>>();
        
        if section_split.len() <= 1 {
            posibilities_per_number.push(1);
            continue;
        }
        
        if section_split.len() > 2 {
            posibilities_per_number.push(actual_probabilities - 1);
            continue;
        }

        if number_section.chars().filter(|&character| character == '#').count() as u64 == number {
            posibilities_per_number.push(1);
            continue;
        }

        actual_probabilities -= (number - 1) - section_split[0].len() as u64;
        actual_probabilities -= (number - 1) - section_split[1].len() as u64;
        
        posibilities_per_number.push(actual_probabilities);
    }
    
    let mut result = 0;
    
    for number_index in 0..posibilities_per_number.len() {
        let number = posibilities_per_number[number_index];

        if number == 0 || number == 1 {
            continue;
        }
        
        if number_index != posibilities_per_number.len() - 1 || posibilities_per_number.len() == 1 {
            result += number;
            continue;
        }
        
        result += (1..number).sum::<u64>();
    }

    if result == 0 {
        return 1;
    }
    
    result
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
        println!("{}", line_arrangements);
        println!("=====================");
        result += line_arrangements;
    });

    result
}