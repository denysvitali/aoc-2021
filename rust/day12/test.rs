#[cfg(test)]
mod tests {
    use crate::{
        part_one, 
        part_two, 
        parse_puzzle_input,
        dfs_paths
    };
    use petgraph::dot;

    #[test]
    fn sample_paths_1(){
        let mut c = parse_puzzle_input("input/sample.txt");
        // println!("{:?}", dot::Dot::with_config(&g, &[dot::Config::EdgeNoLabel]));
        let paths = dfs_paths(&mut c);
        
    }

    #[test]
    fn sample_paths_2(){
        let mut c = parse_puzzle_input("input/sample.txt");
        c.visit_small = 2;
        // println!("{:?}", dot::Dot::with_config(&g, &[dot::Config::EdgeNoLabel]));
        let paths = dfs_paths(&mut c);
        for p in &paths {
            println!("{}", p.join(","));
        }
        assert_eq!(paths.len(), 36);
    }

    #[test]
    fn sample_part_one() {
        assert_eq!(part_one("input/sample.txt"), 10)
    }

    #[test]
    fn sample2_part_one() {
        assert_eq!(part_one("input/sample2.txt"), 19)
    }

    #[test]
    fn sample3_part_one() {
        assert_eq!(part_one("input/sample3.txt"), 226)
    }


    #[test]
    fn sample_part_two() {
        assert_eq!(part_two("input/sample.txt"), 103)
    }

    #[test]
    fn sample2_part_two() {
        assert_eq!(part_two("input/sample2.txt"), 103)
    }

    #[test]
    fn sample3_part_two() {
        assert_eq!(part_two("input/sample3.txt"), 3509)
    }

    #[test]
    fn input_part_one() {
        assert_eq!(part_one("input/input.txt"), 3495)
    }

    #[test]
    fn input_part_two() {
        assert_eq!(part_two("input/input.txt"), 94849)
    }
}