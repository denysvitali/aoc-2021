#[cfg(test)]
mod tests {
    use crate::{Board, part_one, part_two};

    #[test]
    fn board_mark_number() {
        let mut b = Board {
            won: false,
            content: vec![vec![(1, false), (2, false)], vec![(3, false), (4, false)]],
        };

        b.mark(3);
        assert_eq!(b.content, vec![vec![(1, false), (2, false)], vec![(3, true), (4, false)]])
    }

    #[test]
    fn board_won() {
        let mut b = Board {
            won: false,
            content: vec![vec![(1, true), (2, true)], vec![(3, false), (4, false)]],
        };

        assert_eq!(b.won(), true);
        assert_eq!(b.won, true);
    }

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 4512)
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), 1924)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 25023)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), 2634)
    }
}