#[cfg(test)]
mod tests {
    use crate::{parse_input, part_one, part_two};

    #[test]
    fn test_astar(){
        let matrix = parse_input("input/sample.txt");
        let (cost, path) = matrix.route((0, 0), matrix.size);

        matrix.draw_path(&path);

        assert_eq!(cost, 40);
        assert_eq!(path, vec![
            (0, 0), (0, 1), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (6, 3), (7, 3), (7, 4), (8, 4), (8, 5), (8, 6), (8, 7), (8, 8), (9, 8), (9, 9)
        ])
    }

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 40)
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), 0)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 581)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), 0)
    }
}