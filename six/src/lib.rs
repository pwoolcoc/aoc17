use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<u8> {
    let nums = input.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse().expect("Could not parse number"))
                    .collect();
    nums
}

pub fn run(input: &str) -> usize {
    let mut banks = parse_input(input);
    let mut count = 0;
    let mut states: HashSet<Vec<u8>> = HashSet::new();
    loop {
        count += 1;
        println!("{:?}", &banks);
        cycle(&mut banks);
        if states.contains(&banks) {
            return count;
        }
        states.insert(banks.clone());
    }
}

fn idx_of_max(banks: &[u8]) -> usize {
    let m = banks.iter().enumerate().fold((0usize, 0u8), |acc, i| {
        let max_idx = acc.0;
        let max_num = acc.1;
        let idx = i.0;
        let num = *i.1;
        if num > max_num {
            (idx, num)
        } else {
            (max_idx, max_num)
        }
    });
    m.0
}

fn cycle(banks: &mut Vec<u8>) {
    let mut it = (0..(banks.len())).cycle().peekable();
    let idx = idx_of_max(&banks);
    // advance the cyclical iterator to the index of the bank with the most blocks
    loop {
        if let Some(i) = it.peek() {
            if i == &idx {
                break
            }
        }
        it.next();
    }
    let start = it.next().unwrap(); // should be safe since this is a .cycle() iterator
    let num_blocks = banks[start];
    banks[start] = 0;
    for _ in 0..num_blocks {
        let next = it.next().unwrap();
        banks[next] += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "14 0   15  12  11  11  3   5   1   6   8   4   9   1   8   4";
        let num = run(&input);
        println!("answer is: {}", &num);
    }

    #[test]
    fn given_test() {
        let input = "0\t2\t7\t0";
        let num = run(&input);
        assert_eq!(num, 5);
    }
}
