#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 40)
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), -2)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 581)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), -2)
    }
}