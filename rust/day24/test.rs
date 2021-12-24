#[cfg(test)]
mod tests {
    use crate::{exec, parse_input, part_one, part_two};

    #[test]
    fn test_alu_1() {
        let input = parse_input("input/sample-1.txt");
        let exec_result = exec(1, 1, &input);
        assert_eq!(*exec_result.get(&'x').unwrap(), -1)
    }

    #[test]
    fn test_alu_2() {
        let input = parse_input("input/sample-1.txt");
        let exec_result = exec(5, 1, &input);
        assert_eq!(*exec_result.get(&'x').unwrap(), -5)
    }

    #[test]
    fn test_alu_3() {
        let input = parse_input("input/sample-2.txt");
        let exec_result = exec(13, 2, &input);
        assert_eq!(*exec_result.get(&'z').unwrap(), 1)
    }

    #[test]
    fn test_alu_4() {
        let input = parse_input("input/sample-2.txt");
        let exec_result = exec(14, 2, &input);
        assert_eq!(*exec_result.get(&'z').unwrap(), 0)
    }

    #[test]
    fn test_alu_5() {
        let input = parse_input("input/sample-2.txt");
        let exec_result = exec(39, 2, &input);
        assert_eq!(*exec_result.get(&'z').unwrap(), 1)
    }

    #[test]
    fn test_alu_6() {
        let input = parse_input("input/sample-3.txt");
        let exec_result = exec(5, 1, &input);
        assert_eq!(*exec_result.get(&'z').unwrap(), 1)
    }

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample-1.txt"), 40)
    }

    #[test]
    fn sample_2_part_one() {
        assert_eq!(part_one("input/sample-2.txt"), 40)
    }

    #[test]
    fn sample_3_part_one() {
        assert_eq!(part_one("input/sample-3.txt"), 40)
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
