#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 5934)
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), 26984457539)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 351092)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), 1595330616005)
    }
}