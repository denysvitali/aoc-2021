#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 15)
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), 1134)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 554)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), 1017792)
    }
}