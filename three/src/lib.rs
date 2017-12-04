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
move right 1
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

    pub fn build(&mut self, up_to_level: Option<usize>, up_to_num: Option<u64>) {
        self.mark_curr();

        let mut lvl = 0;
        while (up_to_level.is_some() && lvl < up_to_level.unwrap()) || (up_to_num.is_some() && self.num < up_to_num.unwrap()) {
            self.mark_right();

            let up = (lvl * 2) + 1;
            for _ in 0..up {
                self.mark_up();
            }

            let left = (lvl * 2) + 2;
            for _ in 0..left {
                self.mark_left();
            }

            let down = (lvl * 2) + 2;
            for _ in 0..down {
                self.mark_down();
            }

            let right = (lvl * 2) + 2;
            for _ in 0..right {
                self.mark_right();
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

    fn check_max(&self, num: isize) -> bool {
        (num.abs() as usize) > self.max
    }

    fn mark_curr(&mut self) {
        self.cells.push(Cell {
            pos: self.pos,
            val: self.num,
        })
    }

    fn mark_left(&mut self) {
        self.pos = (self.pos.0 - 1, self.pos.1);
        self.num += 1;
        self.mark_curr();
    }

    fn mark_up(&mut self) {
        self.pos = (self.pos.0, self.pos.1 + 1);
        self.num += 1;
        if self.check_max(self.pos.1 + 1) {
            self.max = (self.pos.1 + 1) as usize;
        }
        self.mark_curr();
    }

    fn mark_right(&mut self) {
        self.pos = (self.pos.0 + 1, self.pos.1);
        self.num += 1;
        if self.check_max(self.pos.0 + 1) {
            self.max = (self.pos.0 + 1) as usize;
        }
        self.mark_curr();
    }

    fn mark_down(&mut self) {
        self.pos = (self.pos.0, self.pos.1 - 1);
        self.num += 1;
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
                val: 2,
            },
            Cell {
                pos: (1, 1),
                val: 3,
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
                val: 6,
            },
            Cell {
                pos: (-1, -1),
                val: 7,
            },
            Cell {
                pos: (0, -1),
                val: 8,
            },
            Cell {
                pos: (1, -1),
                val: 9,
            },
            Cell {
                pos: (2, -1),
                val: 10,
            },
            Cell {
                pos: (2, 0),
                val: 11,
            },
            Cell {
                pos: (2, 1),
                val: 12,
            },
            Cell {
                pos: (2, 2),
                val: 13,
            },
            Cell {
                pos: (1, 2),
                val: 14,
            },
            Cell {
                pos: (0, 2),
                val: 15,
            },
            Cell {
                pos: (-1, 2),
                val: 16,
            },
            Cell {
                pos: (-2, 2),
                val: 17,
            },
            Cell {
                pos: (-2, 1),
                val: 18,
            },
            Cell {
                pos: (-2, 0),
                val: 19,
            },
            Cell {
                pos: (-2, -1),
                val: 20,
            },
            Cell {
                pos: (-2, -2),
                val: 21,
            },
            Cell {
                pos: (-1, -2),
                val: 22,
            },
            Cell {
                pos: (0, -2),
                val: 23,
            },
            Cell {
                pos: (1, -2),
                val: 24,
            },
            Cell {
                pos: (2, -2),
                val: 25,
            },
        ];

        let mut g = Grid::new();
        g.build(Some(2), None);

        assert_eq!(should_be, g.cells);

        let mut g = Grid::new();
        g.build(None, Some(1024));
        //println!("{}", g.num);

        assert_eq!(g.get_distance(1), 0);
        assert_eq!(g.get_distance(12), 3);
        assert_eq!(g.get_distance(23), 2);
        assert_eq!(g.get_distance(1024), 31);

        let mut g = Grid::new();
        g.build(None, Some(347991));
        println!("solution is: {}", g.get_distance(347991));
    }
}
