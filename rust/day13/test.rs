#[cfg(test)]
mod tests {
    use crate::{Coord, part_one, part_two};

    #[test]
    fn test_fold() {
        // (2,14) goes to (2, 0) when folding over (0, 7)
        // (0, 8) goes to (0, 6) when folding over y=7
        // (0, 9) goes to (0, 5) when folding over y=7
        let f = Coord { x: 0, y: 7 };
        let a = Coord { x: 2, y: 14 };
        let b = Coord { x: 0, y: 8 };
        let c = Coord { x: 1, y: 1 };

        assert_eq!(a.fold(f), Coord { x: 2, y: 0 });
        assert_eq!(b.fold(f), Coord { x: 0, y: 6 });
        assert_eq!(c.fold(f), Coord { x: 1, y: 1 });

        // Folding across X
        let f2 = Coord { x: 5, y: 0 };
        let d = Coord { x: 6, y: 0 };
        let e = Coord { x: 9, y: 0 };
        assert_eq!(d.fold(f2), Coord { x: 4, y: 0 });
        assert_eq!(e.fold(f2), Coord { x: 1, y: 0 });
    }

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 17)
    }

    #[test]
    fn sample_part_two() {
        // Should be:
        /*
            ▓▓▓▓▓
            ▓   ▓
            ▓   ▓
            ▓   ▓
            ▓▓▓▓▓
         */
        assert_eq!(part_two("input/sample.txt"), -1)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 729)
    }

    #[test]
    fn input_part_two() {
        /*
            ▓▓▓   ▓▓  ▓▓▓▓ ▓    ▓▓▓  ▓  ▓ ▓▓▓▓ ▓▓▓
            ▓  ▓ ▓  ▓    ▓ ▓    ▓  ▓ ▓  ▓ ▓    ▓  ▓
            ▓  ▓ ▓      ▓  ▓    ▓▓▓  ▓▓▓▓ ▓▓▓  ▓  ▓
            ▓▓▓  ▓ ▓▓  ▓   ▓    ▓  ▓ ▓  ▓ ▓    ▓▓▓
            ▓ ▓  ▓  ▓ ▓    ▓    ▓  ▓ ▓  ▓ ▓    ▓
            ▓  ▓  ▓▓▓ ▓▓▓▓ ▓▓▓▓ ▓▓▓  ▓  ▓ ▓    ▓
         */
        assert_eq!(part_two("input/input.txt"), -1)
    }
}