use std::fmt;

#[derive(Clone, PartialEq)]
struct SpinLock {
    buf: Vec<u64>,
    num: u64,
    cur: usize,
    step: usize,
}

impl fmt::Debug for SpinLock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.buf.iter().enumerate().map(|(ref i, ref s)| {
            if *i == self.cur {
                format!("({})", s)
            } else {
                format!("{}", s)
            }
        }).collect::<Vec<_>>().join(" "))
    }
}

impl SpinLock {
    fn new(step: usize) -> SpinLock {
        let mut v = Vec::with_capacity(2018);
        v.insert(0, 0);
        SpinLock {
            buf: v,
            cur: 0,
            num: 0,
            step: step,
        }
    }

    fn advance(&mut self) -> Result<(), ()> {
        let new_cur = ((self.cur + self.step) % self.buf.len()) + 1;
        self.num += 1;
        self.cur = new_cur;
        self.buf.insert(new_cur, self.num);
        Ok(())
    }

    fn next(&self) -> Option<u64> {
        if self.cur == self.buf.len() - 1 {
            None
        } else {
            Some(self.buf[self.cur + 1])
        }
    }
}

fn run(a: usize) -> u64 {
    let mut s = SpinLock::new(a);
    for i in 0..2017 {
        s.advance();
    }
    s.next().expect("WHOOPS")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(run(3), 638);
    }

    #[test]
    fn answer() {
        println!("{}", run(324));
    }
}
