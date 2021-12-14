#[cfg(test)]
mod tests {
    use crate::{part_one, part_two, get_template_and_pairs};

    #[test]
    fn test_parse_input(){
        let (t, p) = get_template_and_pairs("input/sample.txt");
        assert_eq!(t, "NNCB");
        assert_eq!(p.len(), 16)
    }

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 1588)
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), 2188189693529)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 3230)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), 3542388214529)
    }
}