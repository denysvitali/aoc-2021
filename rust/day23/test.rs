#[cfg(test)]
mod tests {
    use crate::{Amber, Bronze, Copper, Desert, GameState, Move, parse_input, part_one, part_two};

    #[test]
    fn check_win(){
        let game_state = parse_input("input/win.txt");
        assert_eq!(true, game_state.win())
    }

    #[test]
    fn next_moves(){
        let game_state = parse_input("input/sample.txt");
        let next_moves = game_state.next_moves();
        println!("next_moves={:?}", next_moves);
    }

    #[test]
    fn next_moves_2(){
        let game_state = parse_input("input/sample-2.txt");
        let next_moves = game_state.next_moves();
        for m in next_moves {
            if m.whom != Copper {
                continue
            }
            let gs = game_state.clone().play_move(&m);
            println!("move = {:?}, game state: \n{}", m, gs);
        }
    }

    #[test]
    fn next_moves_3(){
        let game_state = parse_input("input/sample-3.txt");
        let next_moves = game_state.next_moves();
        for m in next_moves {
            if m.whom != Copper {
                continue
            }
            let gs = game_state.clone().play_move(&m);
            println!("move = {:?}, game state: \n{}", m, gs);
        }
    }

    #[test]
    fn check_not_won(){
        let game_state = parse_input("input/sample.txt");
        assert_eq!(false, game_state.win())
    }

    #[test]
    fn check_not_won2(){
        let game_state = parse_input("input/not-won.txt");
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
    fn check_right_column(){
        let game_state = parse_input("input/win.txt");
        assert_eq!(true, game_state.correct_column(&(3,1)));
        assert_eq!(true, game_state.correct_column(&(5,1)));
        assert_eq!(true, game_state.correct_column(&(7,1)));
    }

    #[test]
    fn check_wrong_column(){
        let game_state = parse_input("input/sample.txt");
        assert_eq!(false, game_state.correct_column(&(3,1)));
        assert_eq!(false, game_state.correct_column(&(5,1)));
        assert_eq!(false, game_state.correct_column(&(7,1)));
    }

    #[test]
    fn check_move_action(){
        let game_state = parse_input("input/sample.txt");
        let game_state = game_state.play_move(&Move {
            from: (7, 2),
            to: (4, 1),
            whom: Bronze,
            cost: 40
        });
        println!("game_state=\n{}", game_state)
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
