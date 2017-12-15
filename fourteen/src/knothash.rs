use std::fmt;
use std::convert::From;

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

#[derive(Debug, PartialEq, Clone)]
pub struct KnotHash(String);

impl fmt::Display for KnotHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<'a> From<&'a str> for KnotHash {
    fn from(s: &'a str) -> KnotHash {
        KnotHash::new(s)
    }
}

impl KnotHash {
    pub fn new(input: &str) -> KnotHash {
        let mut list = List::new();
        let lengths = get_lengths(input);
        for _ in 0..64 {
            run_one(&lengths, &mut list);
        }
        let dense = get_dense_hash(&list);
        let ashex = dense.iter().map(|d| format!("{:02x}", d)).collect::<Vec<_>>();
        KnotHash(ashex.join(""))
    }

    pub fn to_bin_str(&self) -> String {
        let mut accum = String::new();
        let hex = &self.0;
        for i in 0..hex.len() {
            let j = i + 1;
            let num: u64 = u64::from_str_radix(&hex[i..j], 16).expect("Couldn't get a number from hex string");
            accum = format!("{}{:04b}", accum, num);
        }
        accum
    }
}

