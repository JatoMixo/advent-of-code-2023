use day_04::get_total_value;

fn main() {
    let input = std::fs::read_to_string("cards.txt").expect("Forgot the file :/");

    println!("{}", get_total_value(input));
}