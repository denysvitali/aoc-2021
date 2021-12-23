#[cfg(test)]
mod tests {
    use crate::{Amber, Bronze, Copper, Desert, GameState, parse_input, part_one, part_two};

    #[test]
    fn check_win(){
        let game_state = parse_input("input/win.txt");
        assert_eq!(true, game_state.win())
    }

    #[test]
    fn check_not_win(){
        let game_state = parse_input("input/sample.txt");
        assert_eq!(false, game_state.win())
    }

    #[test]
    fn check_right_position(){
        let game_state = parse_input("input/sample.txt");
        assert_eq!(true, game_state.in_right_position(&(3,2), &Amber));
        assert_eq!(true, game_state.in_right_position(&(3,3), &Amber));
        assert_eq!(false, game_state.in_right_position(&(3,2), &Bronze));
        assert_eq!(false, game_state.in_right_position(&(3,3), &Bronze));
        assert_eq!(false, game_state.in_right_position(&(3,2), &Copper));
        assert_eq!(false, game_state.in_right_position(&(3,3), &Copper));
        assert_eq!(false, game_state.in_right_position(&(3,2), &Desert));
        assert_eq!(false, game_state.in_right_position(&(3,3), &Desert));

        assert_eq!(true, game_state.in_right_position(&(5,2), &Bronze));
        assert_eq!(true, game_state.in_right_position(&(5,3), &Bronze));
        assert_eq!(false, game_state.in_right_position(&(5,2), &Amber));
        assert_eq!(false, game_state.in_right_position(&(5,3), &Amber));

        assert_eq!(true, game_state.in_right_position(&(7,2), &Copper));
        assert_eq!(true, game_state.in_right_position(&(7,3), &Copper));

        assert_eq!(true, game_state.in_right_position(&(9,2), &Desert));
        assert_eq!(true, game_state.in_right_position(&(9,3), &Desert));

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
