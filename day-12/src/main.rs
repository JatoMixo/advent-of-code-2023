use day_12::calculate_multiple_arrangements;
fn main() {
    let input = std::fs::read_to_string("arrangements.txt").expect("The file");

    println!("{}", calculate_multiple_arrangements(input));
}