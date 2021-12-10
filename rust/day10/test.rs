#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::{part_one, part_two, check_line, identify_token, Token, Incomplete, Corrupted};

    #[test]
    fn test_check_line_1(){
        let res = check_line(
            &"[({(<(())[]>[[{[]{<()<>>"
                .chars()
                .map(identify_token)
                .collect::<Vec<Token>>()
        );
        let mut stack : VecDeque<Token> = VecDeque::new();
        "}}]])})]".chars()
            .map(identify_token)
            .for_each(|x| {
                stack.push_back(x.inverse())
            });
        assert_eq!(res, Incomplete(stack.into()));
    }

    #[test]
    fn test_check_line_5(){
        let res = check_line(
            &"{([(<{}[<>[]}>{[]{[(<()>"
                .chars()
                .map(identify_token)
                .collect::<Vec<Token>>()
        );
        assert_eq!(res, Corrupted(Token::ClsCurly));
    }

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 26397)
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), 288957)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 344193)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), 3241238967)
    }
}