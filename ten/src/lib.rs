#![allow(dead_code)]

#[derive(Debug)]
struct List {
    curr: usize,
    skip_size: usize,
    list: Vec<i64>,
}

impl List {
    fn new() -> List {
        List::with_list((0..256).collect::<Vec<_>>())
    }

    fn with_list<S: Into<Vec<i64>>>(s: S) -> List {
        List {
            curr: 0,
            skip_size: 0,
            list: s.into(),
        }
    }

    fn step_wrap(&mut self, len: usize) -> Result<(), ()> {
        let right_list = self.list[self.curr..].to_vec();
        let pivot = right_list.len();
        let left_len = len - pivot;
        let left_list = self.list[0..left_len].to_vec();
        let mut v = right_list.to_vec();
        v.extend_from_slice(&left_list);
        v.reverse();
        let (left, right) = v.split_at(pivot);
        for (i, elem) in left.iter().enumerate() {
            self.list[self.curr + i] = *elem;
        }
        for (i, elem) in right.iter().enumerate() {
            self.list[i] = *elem;
        }
        Ok(())
    }

    fn step(&mut self, len: usize) -> Result<(), ()> {
        if len == 0 || len == 1 {
            return Ok(());
        }

        if len + self.curr > self.list.len() {
            Ok(self.step_wrap(len)?)
        } else {
            let mut s = self.list[self.curr..(self.curr + len)].to_vec();
            s.reverse();
            for (i, elem) in s.iter().enumerate() {
                self.list[self.curr + i] = *elem;
            }
            println!("{:?}", self.list);
            Ok(())
        }
    }
}

fn run(input: &str, list: &mut List) -> i64 {
    let lengths = input.trim().split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("NOT A NUMBER"))
        .collect::<Vec<usize>>();
    for len in lengths {
        list.step(len).expect("Couldnt do step");
        list.curr += len + list.skip_size;
        list.curr = list.curr % list.list.len();
        list.skip_size += 1;
    }
    list.list[0] * list.list[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut l = List::new();
        let input = "199,0,255,136,174,254,227,16,51,85,1,2,22,17,7,192";
        println!("answer is {}", run(input, &mut l));
    }

    /*
    #[test]
    fn test_step_wrap() {
        let mut l = List::new();
        l.curr = 250;
        l.step(10);
        assert_eq!(&l.list[0..5], &[253, 252, 251, 250, 4]);
        assert_eq!(&l.list[250..], &[3, 2, 1, 0, 255, 254]);
    }

    #[test]
    fn it_works() {
        let mut l = List::with_list(vec![0i64, 1, 2, 3, 4]);
        let input = "3,4,1,5";
        assert_eq!(run(&input, &mut l), 12);
    }
    */
}
