const NUM: u64 = 2147483647;
const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Generator {
    valu: u64,
    factor: u64,
    criteria: u64,
}

impl Generator {
    fn new(value: u64, factor: u64, criteria: u64) -> Generator {
        Generator {
            valu: value,
            factor: factor,
            criteria: criteria,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(loop {
            let interm = self.valu * self.factor;
            self.valu = interm % NUM;
            if self.valu % self.criteria == 0 {
                break self.valu
            }
        })
    }
}

fn low16bits(num: u64) -> u64 {
    num & 0xffff
}

fn run(astart: u64, acrit: u64, bstart: u64, bcrit: u64, rounds: usize) -> usize {
    let a = Generator::new(astart, A_FACTOR, acrit);
    let b = Generator::new(bstart, B_FACTOR, bcrit);
    let iter = a.zip(b);
    let count = iter.take(rounds).map(|(a, b)| (low16bits(a), low16bits(b))).filter(|&(a, b)| a == b).count();
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(run(65, 4, 8921, 8, 5_000_000), 309);
    }

    #[test]
    fn answer() {
        println!("answer is {}", run(591, 4, 393, 8, 5_000_000));
    }
}
