#![allow(dead_code)]

use std::cell::RefCell;
use std::fmt;
use std::convert::From;
use std::ops::Index;

use knothash::KnotHash;

mod knothash;

struct GridCell {
    value: bool,
    group: RefCell<Option<usize>>,
}

impl fmt::Debug for GridCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let g = self.group();
        write!(f, "{:04}", if g == -1 { "_".to_string() } else { g.to_string() })
    }
}

impl fmt::Display for GridCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<char> for GridCell {
    fn from(c: char) -> GridCell {
        if c == '1' {
            GridCell {
                value: true,
                group: RefCell::new(None),
            }
        } else if c == '0' {
            GridCell {
                value: false,
                group: RefCell::new(None),
            }
        } else {
            panic!("WHAT IS THIS");
        }
    }
}

impl GridCell {
    fn is_set(&self) -> bool {
        self.value
    }

    fn has_group(&self) -> bool {
        self.group.borrow().is_some()
    }

    fn set_group(&self, group: usize) {
        *self.group.borrow_mut() = Some(group);
    }

    fn group(&self) -> isize {
        match *self.group.borrow() {
            Some(g) => g as isize,
            None => -1isize,
        }
    }
}

struct BinGrid {
    data: Vec<GridCell>,
    width: usize,
    height: usize,
}

impl fmt::Debug for BinGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num_rows = self.rows();
        for i in 0..num_rows {
            let start = i * self.width;
            let end = start + self.width;
            let row = self.data[start..end].iter().fold(String::new(), |accum, s| format!("{} {}", accum, s));
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for BinGrid {
    type Output = GridCell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1)
    }
}

impl<'a> From<&'a str> for BinGrid {
    fn from(input: &'a str) -> BinGrid {
        let mut accum = vec![];
        for i in 0..128 {
            let inp = format!("{}-{}", input, i);
            let hash = KnotHash::from(&inp[..]);
            for cell in hash.to_bin_str().chars().map(|c| GridCell::from(c)) {
                accum.push(cell);
            }
        }
        BinGrid {
            data: accum,
            width: 128,
            height: 128,
        }
    }
}

impl BinGrid {
    fn rows(&self) -> usize {
        (self.data.len() / self.width) as usize
    }

    fn cols(&self) -> usize {
        self.width
    }

    fn cells(&self) -> usize {
        self.data.len()
    }

    // starts with 0, 0 in the top-left corner
    fn coords_to_num(&self, x: usize, y: usize) -> usize {
        // y is the row we're in
        let row_start = y * self.width;
        row_start + x
    }

    fn is_left_edge(&self, x: usize, _: usize) -> bool {
        x == 0
    }

    fn is_right_edge(&self, x: usize, _: usize) -> bool {
        x == (self.width - 1)
    }

    fn is_top_edge(&self, _: usize, y: usize) -> bool {
        y == 0
    }

    fn is_bottom_edge(&self, _: usize, y: usize) -> bool {
        y == (self.height - 1)
    }

    fn left(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if self.is_left_edge(x, y) {
            None
        } else {
            Some((x - 1, y))
        }
    }

    fn right(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if self.is_right_edge(x, y) {
            None
        } else {
            Some((x + 1, y))
        }
    }

    fn top(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if self.is_top_edge(x, y) {
            None
        } else {
            Some((x, y - 1))
        }
    }

    fn bottom(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if self.is_bottom_edge(x, y) {
            None
        } else {
            Some((x, y + 1))
        }
    }

    fn get(&self, x: usize, y: usize) -> &GridCell {
        let idx = self.coords_to_num(x, y);
        &self.data[idx]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut GridCell {
        let idx = self.coords_to_num(x, y);
        &mut self.data[idx]
    }

    fn mark_group(&self, x: usize, y: usize, group: usize) -> bool {
        let cell = self.get(x, y);
        if !cell.is_set() {
            return false;
        }
        if cell.has_group() {
            println!("cell({}, {}) already set to {}", x, y, cell.group());
            return false;
        }
        {
            println!("setting cell({}, {}) to {}", x, y, group);
            cell.set_group(group);
        }
        if let Some(coords) = self.left(x, y) {
            self.mark_group(coords.0, coords.1, group);
        }
        if let Some(coords) = self.right(x, y) {
            self.mark_group(coords.0, coords.1, group);
        }
        if let Some(coords) = self.top(x, y) {
            self.mark_group(coords.0, coords.1, group);
        }
        if let Some(coords) = self.bottom(x, y) {
            self.mark_group(coords.0, coords.1, group);
        }
        true
    }
}

fn run(input: &str) -> usize {
    let mut group = 0;
    let grid = BinGrid::from(input);
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if grid.mark_group(x, y, group) {
                group += 1;
            }
        }
    }
    group
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "nbysizxe";
        println!("answer is {}", run(input));
    }
}
