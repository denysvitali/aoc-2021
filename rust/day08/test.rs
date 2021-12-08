#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 26)
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), 61229)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 365)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), 975706)
    }
}