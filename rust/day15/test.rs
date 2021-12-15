#[cfg(test)]
mod tests {
    use crate::{parse_input, part_one, part_two, Coord};

    #[test]
    fn test_astar() {
        let matrix = parse_input("input/sample.txt");
        let (cost, path) = matrix.route((0, 0), (matrix.size.0 - 1, matrix.size.1 - 1));

        matrix.draw_path(&path);

        assert_eq!(cost, 40);
    }

    #[test]
    fn big_map() {
        let mut matrix = parse_input("input/sample.txt");
        let first_line = "11637517422274862853338597396444961841755517295286";
        let first_row = first_line
            .chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let last_line = "67554889357866599146897761125791887223681299833479";
        let last_row = last_line
            .chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for i in 0..matrix.size.0 * 5 {
            assert_eq!(matrix.get_value((i, 0)), first_row[i]);
        }

        for i in 0..matrix.size.0 * 5 {
            assert_eq!(matrix.get_value((i, matrix.size.1 * 5 - 1)), last_row[i]);
        }
        println!()
    }

    #[test]
    fn test_astar2() {
        let mut matrix = parse_input("input/sample.txt");
        matrix.real_size = (matrix.size.0, matrix.size.1);
        matrix.size = (matrix.real_size.0 * 5, matrix.real_size.1 * 5);
        let end = (matrix.size.0 - 1, matrix.size.1 - 1);
        let (cost, path) = matrix.route((0, 0), end);

        assert_eq!(2, matrix.get_value((10, 0)));
        assert_eq!(2, matrix.get_value((0, 10)));
        assert_eq!(6, matrix.get_value((49, 0)));

        matrix.draw_path(&path);

        assert_eq!(cost, 315);
    }

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 40)
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), 315)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 581)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), 2916)
    }
}
