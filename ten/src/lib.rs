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
            Ok(())
        }
    }
}

fn get_lengths(input: &str) -> Vec<usize> {
    let mut lengths = input.chars()
        .map(|c| c as u32 as usize)
        .collect::<Vec<usize>>();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    lengths
}

fn run_one(lengths: &[usize], list: &mut List) -> i64 {
    for len in lengths {
        list.step(*len).expect("Couldnt do step");
        list.curr += len + list.skip_size;
        list.curr = list.curr % list.list.len();
        list.skip_size += 1;
    }
    list.list[0] * list.list[1]
}

fn xor(nums: &[i64]) -> i64 {
    nums.iter()
        .fold(0, |acc, x| {
                acc ^ x
        })
}

fn get_dense_hash(list: &List) -> Vec<i64> {
    (0..16).map(|i| {
                let start = i * 16;
                let end = start + 16;
                xor(&list.list[start..end])
            })
            .collect()
}

fn to_hex(num: i64) -> String {
    format!("{:02x}", num)
}

fn run(input: &str, list: &mut List) -> String {
    let lengths = get_lengths(input);
    for _ in 0..64 {
        run_one(&lengths, list);
    }
    let dense = get_dense_hash(&list);
    let ashex = dense.iter().map(|d| to_hex(*d)).collect::<Vec<_>>();
    ashex.join("")
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
        let mut l = List::new();
        let input = "";
        assert_eq!(run(&input, &mut l), "a2582a3a0e66e6e86e3812dcb672a272");
        let mut l = List::new();
        let input = "AoC 2017";
        assert_eq!(run(&input, &mut l), "33efeb34ea91902bb2f59c9920caa6cd");
        let mut l = List::new();
        let input = "1,2,3";
        assert_eq!(run(&input, &mut l), "3efbe78a8d82f29979031a4aa0b16a9d");
        let mut l = List::new();
        let input = "1,2,4";
        assert_eq!(run(&input, &mut l), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
    */
}
