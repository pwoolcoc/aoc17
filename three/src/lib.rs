/*
1: (0, 0)
2: (1, 0)
3: (1, 1)
4: (0, 1)
5: (-1, 1)
6: (-1, 0)
7: (-1, -1)
8: (0, -1)
9: (1, -1)

    -3 -2 -1  0  1  2 3
    --------------------
  3|37 36 35 34 33 32 31
  2|38 17 16 15 14 13 30
  1|39 18 5  4  3  12 29
  0|40 19 6  1  2  11 28
 -1|41 20 7  8  9  10 27
 -2|42 21 22 23 24 25 26
 -3|43 44 45 46 47 48 49


level: 0
go right 1
go up (($level * 2) + 1)
go left (($level * 2) + 2)
go down (($level * 2) + 2)
go right (($level * 2) + 2)
level: 1
go right 1
go up (($level * 2) + 1)
go left (($level * 2) + 2)
go down (($level * 2) + 2)
go right (($level * 2) + 2)
level: 2
...etc

*/

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pos: (isize, isize),
    val: u64,
}

impl ::std::fmt::Display for Cell {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}: ({}, {})", self.val, self.pos.0, self.pos.1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pos: (isize, isize),
    num: u64,
    max: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            pos: (0, 0),
            num: 1,
            max: 0,
            cells: vec![],
        }
    }

    fn under_level(&self, lvl: usize, up_to_lvl: Option<usize>) -> bool {
        match up_to_lvl {
            None => true,
            Some(l) => lvl < l
        }
    }

    pub fn build(&mut self, up_to_level: Option<usize>, up_to_num: u64) {
        self.mark_curr();

        let mut lvl = 0;
        while self.under_level(lvl, up_to_level) && self.num < up_to_num {
            self.mark_right(up_to_num);

            let up = (lvl * 2) + 1;
            for _ in 0..up {
                self.mark_up(up_to_num);
            }

            let left = (lvl * 2) + 2;
            for _ in 0..left {
                self.mark_left(up_to_num);
            }

            let down = (lvl * 2) + 2;
            for _ in 0..down {
                self.mark_down(up_to_num);
            }

            let right = (lvl * 2) + 2;
            for _ in 0..right {
                self.mark_right(up_to_num);
            }

            lvl += 1;
        }
    }

    pub fn get_cell_with_val(&self, num: usize) -> &Cell {
        debug_assert!(num > 0);
        let idx = num - 1;
        &self.cells[idx]
    }

    pub fn get_distance(&self, num: usize) -> u64 {
        let cell = self.get_cell_with_val(num);
        let abs_x = cell.pos.0.abs() as u64;
        let abs_y = cell.pos.1.abs() as u64;
        abs_x + abs_y
    }

    fn get_val_for_cell(&self, pos: (isize, isize)) -> Option<u64> {
        // definitely could do this better
        for cell in &self.cells {
            if pos == cell.pos {
                return Some(cell.val)
            }
        }
        None
    }

    fn get_val_for_left(&self, pos: (isize, isize)) -> Option<u64> {
        self.get_val_for_cell((pos.0 - 1, pos.1))
    }

    fn get_val_for_up(&self, pos: (isize, isize)) -> Option<u64> {
        self.get_val_for_cell((pos.0, pos.1 + 1))
    }

    fn get_val_for_right(&self, pos: (isize, isize)) -> Option<u64> {
        self.get_val_for_cell((pos.0 + 1, pos.1))
    }

    fn get_val_for_down(&self, pos: (isize, isize)) -> Option<u64> {
        self.get_val_for_cell((pos.0, pos.1 - 1))
    }

    fn get_val_for_left_up(&self, pos: (isize, isize)) -> Option<u64> {
        self.get_val_for_cell((pos.0 - 1, pos.1 + 1))
    }

    fn get_val_for_right_up(&self, pos: (isize, isize)) -> Option<u64> {
        self.get_val_for_cell((pos.0 + 1, pos.1 + 1))
    }

    fn get_val_for_left_down(&self, pos: (isize, isize)) -> Option<u64> {
        self.get_val_for_cell((pos.0 - 1, pos.1 - 1))
    }

    fn get_val_for_right_down(&self, pos: (isize, isize)) -> Option<u64> {
        self.get_val_for_cell((pos.0 + 1, pos.1 - 1))
    }

    fn get_val(&self, max_val: u64) -> u64 {
        let mut val = 0;
        if let Some(left) = self.get_val_for_left(self.pos) {
            val += left;
        }
        if let Some(up) = self.get_val_for_up(self.pos) {
            val += up;
        }
        if let Some(right) = self.get_val_for_right(self.pos) {
            val += right;
        }
        if let Some(down) = self.get_val_for_down(self.pos) {
            val += down;
        }
        if let Some(left_up) = self.get_val_for_left_up(self.pos) {
            val += left_up;
        }
        if let Some(right_up) = self.get_val_for_right_up(self.pos) {
            val += right_up;
        }
        if let Some(left_down) = self.get_val_for_left_down(self.pos) {
            val += left_down;
        }
        if let Some(right_down) = self.get_val_for_right_down(self.pos) {
            val += right_down;
        }
        if val > max_val {
            println!("val {} is over {}", val, max_val);
        }
        val
    }

    fn check_max(&self, num: isize) -> bool {
        (num.abs() as usize) > self.max
    }

    fn mark_curr(&mut self) {
        self.cells.push(Cell {
            pos: self.pos,
            val: self.num,
        })
    }

    fn mark_left(&mut self, max_val: u64) {
        self.pos = (self.pos.0 - 1, self.pos.1);
        self.num = self.get_val(max_val);
        self.mark_curr();
    }

    fn mark_up(&mut self, max_val: u64) {
        self.pos = (self.pos.0, self.pos.1 + 1);
        self.num = self.get_val(max_val);
        if self.check_max(self.pos.1 + 1) {
            self.max = (self.pos.1 + 1) as usize;
        }
        self.mark_curr();
    }

    fn mark_right(&mut self, max_val: u64) {
        self.pos = (self.pos.0 + 1, self.pos.1);
        self.num = self.get_val(max_val);
        if self.check_max(self.pos.0 + 1) {
            self.max = (self.pos.0 + 1) as usize;
        }
        self.mark_curr();
    }

    fn mark_down(&mut self, max_val: u64) {
        self.pos = (self.pos.0, self.pos.1 - 1);
        self.num = self.get_val(max_val);
        self.mark_curr();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let should_be = vec![
            Cell {
                pos: (0, 0),
                val: 1,
            },
            Cell {
                pos: (1, 0),
                val: 1,
            },
            Cell {
                pos: (1, 1),
                val: 2,
            },
            Cell {
                pos: (0, 1),
                val: 4,
            },
            Cell {
                pos: (-1, 1),
                val: 5,
            },
            Cell {
                pos: (-1, 0),
                val: 10,
            },
            Cell {
                pos: (-1, -1),
                val: 11,
            },
            Cell {
                pos: (0, -1),
                val: 23,
            },
            Cell {
                pos: (1, -1),
                val: 25,
            },
            Cell {
                pos: (2, -1),
                val: 26,
            },
            Cell {
                pos: (2, 0),
                val: 54,
            },
            Cell {
                pos: (2, 1),
                val: 57,
            },
            Cell {
                pos: (2, 2),
                val: 59,
            },
            Cell {
                pos: (1, 2),
                val: 122,
            },
            Cell {
                pos: (0, 2),
                val: 133,
            },
            Cell {
                pos: (-1, 2),
                val: 142,
            },
            Cell {
                pos: (-2, 2),
                val: 147,
            },
            Cell {
                pos: (-2, 1),
                val: 304,
            },
            Cell {
                pos: (-2, 0),
                val: 330,
            },
            Cell {
                pos: (-2, -1),
                val: 351,
            },
            Cell {
                pos: (-2, -2),
                val: 362,
            },
            Cell {
                pos: (-1, -2),
                val: 747,
            },
            Cell {
                pos: (0, -2),
                val: 806,
            },
            Cell {
                pos: (1, -2),
                val: 880,
            },
            Cell {
                pos: (2, -2),
                val: 931,
            },
        ];

        let mut g = Grid::new();
        g.build(Some(2), 932);

        assert_eq!(should_be, g.cells);

        let mut g = Grid::new();
        g.build(None, 347992);
    }
}
