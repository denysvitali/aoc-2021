#[cfg(test)]
mod tests {
    use crate::{parse_input, part_one, part_two, u64_to_digits, Executor, Vars};

    #[test]
    fn test_i32_to_digits() {
        assert_eq!(u64_to_digits(123), vec![1, 2, 3]);
        assert_eq!(u64_to_digits(1), vec![1]);
        assert_eq!(u64_to_digits(555), vec![5, 5, 5]);
        assert_eq!(u64_to_digits(1024), vec![1, 0, 2, 4]);
    }

    #[test]
    fn test_alu_1() {
        let input = parse_input("input/sample-1.txt");
        let mut e = Executor::new();
        let exec_result = e.exec(&[1], &Vars::new(), &input, 0);
        assert_eq!(*exec_result.content.get(&'x').unwrap(), -1)
    }

    #[test]
    fn test_alu_2() {
        let input = parse_input("input/sample-1.txt");
        let mut e = Executor::new();
        let exec_result = e.exec(&[5], &Vars::new(), &input, 0);
        assert_eq!(*exec_result.content.get(&'x').unwrap(), -5)
    }

    #[test]
    fn test_alu_3() {
        let input = parse_input("input/sample-2.txt");
        let mut e = Executor::new();
        let exec_result = e.exec(&[1, 3], &Vars::new(), &input, 0);
        assert_eq!(*exec_result.content.get(&'z').unwrap(), 1)
    }

    #[test]
    fn test_alu_4() {
        let input = parse_input("input/sample-2.txt");
        let mut e = Executor::new();
        let exec_result = e.exec(&[1, 4], &Vars::new(), &input, 0);
        assert_eq!(*exec_result.content.get(&'z').unwrap(), 0)
    }

    #[test]
    fn test_alu_5() {
        let input = parse_input("input/sample-2.txt");
        let mut e = Executor::new();
        let exec_result = e.exec(&[3, 9], &Vars::new(), &input, 0);
        assert_eq!(*exec_result.content.get(&'z').unwrap(), 1)
    }

    #[test]
    fn test_alu_6() {
        let input = parse_input("input/sample-3.txt");
        let mut e = Executor::new();
        let exec_result = e.exec(&[5], &Vars::new(), &input, 0);
        assert_eq!(*exec_result.content.get(&'z').unwrap(), 1)
    }

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample-1.txt"), -1)
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
