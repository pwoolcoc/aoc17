fn num_steps(range: u32) -> u32 {
    (range - 1) * 2
}

fn position(delay: u32, depth: u32, range: u32) -> u32 {
    let num_steps = num_steps(range);
    (delay + num_steps + depth) % num_steps
}

fn is_zero(delay: u32, pair: &(u32, u32)) -> bool {
    position(delay, pair.0, pair.1) == 0
}

fn run(input: &str) -> u32 {
    let layer_input = input.trim()
                           .lines()
                           .map(|s| s.trim())
                           .filter(|s| !s.is_empty())
                           .map(|s| s.split(": "))
                           .map(|mut spl| (spl.next().expect("didn't get left side").parse::<u32>().expect("Couldn't parse left side"),
                                           spl.next().expect("didn't get right side").parse::<u32>().expect("Couldn't parse right side")))
                           .collect::<Vec<_>>();
    layer_input.iter().filter(|p| is_zero(0, p)).map(|p| p.0 * p.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"
            0: 3
            1: 2
            4: 4
            6: 4
        "#;
        assert_eq!(run(&input), 24);
    }

    /*
    #[test]
    fn test_answer() {
        let input = r#"
            0: 3
            1: 2
            2: 4
            4: 6
            6: 5
            8: 8
            10: 6
            12: 4
            14: 8
            16: 6
            18: 8
            20: 8
            22: 6
            24: 8
            26: 9
            28: 12
            30: 8
            32: 14
            34: 10
            36: 12
            38: 12
            40: 10
            42: 12
            44: 12
            46: 12
            48: 12
            50: 14
            52: 12
            54: 14
            56: 12
            60: 14
            62: 12
            64: 14
            66: 14
            68: 14
            70: 14
            72: 14
            74: 14
            78: 26
            80: 18
            82: 17
            86: 18
            88: 14
            96: 18
        "#;
        println!("{}", run(&input));
    }
    */
}
