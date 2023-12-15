use day_03::calculate_schematic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(calculate_schematic("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string()), 4361);
    }

    #[test]
    fn the_definitve_1() {
        assert_eq!(calculate_schematic(std::fs::read_to_string("engine_schematic.txt").expect("Nice you fucked up the file again")), 543867);
    }
}