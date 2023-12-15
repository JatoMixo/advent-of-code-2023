use day_03::calculate_schematic;

fn main() {
    let input = std::fs::read_to_string("engine_schematic.txt").expect("Forgot the file...");

    println!("{}", calculate_schematic(input));
}
