const NUM: u64 = 2147483647;
const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Generator {
    valu: u64,
    factor: u64,
}

impl Generator {
    fn new(value: u64, factor: u64) -> Generator {
        Generator {
            valu: value,
            factor: factor,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let interm = self.valu * self.factor;
        self.valu = interm % NUM;
        Some(self.valu)
    }
}

fn low16bits(num: u64) -> u64 {
    num & 0xffff
}

fn run(astart: u64, bstart: u64, rounds: usize) -> usize {
    let a = Generator::new(astart, A_FACTOR);
    let b = Generator::new(bstart, B_FACTOR);
    let iter = a.zip(b);
    let count = iter.take(rounds).map(|(a, b)| (low16bits(a), low16bits(b))).filter(|&(a, b)| a == b).count();
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(run(65, 8921, 40_000_000), 588);
    }

    #[test]
    fn answer() {
        println!("answer is {}", run(591, 393, 40_000_000));
    }
}
