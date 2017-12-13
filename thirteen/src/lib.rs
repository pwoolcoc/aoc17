use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq)]
struct Layer {
    depth: usize,
    cur: usize,
    dir: Direction,
}

impl Layer {
    fn new(depth: usize) -> Layer {
        Layer {
            depth: depth,
            cur: 1,
            dir: Direction::Down,
        }
    }

    fn tick(&mut self) {
        match self.dir {
            Direction::Down => {
                if self.cur == self.depth {
                    self.cur -= 1;
                    self.dir = Direction::Up;
                } else {
                    self.cur += 1;
                }
            },
            Direction::Up => {
                if self.cur == 1 {
                    self.cur += 1;
                    self.dir = Direction::Down;
                } else {
                    self.cur -= 1;
                }
            }
        }
    }

    fn at_top(&self) -> bool {
        self.cur == 1
    }
}

struct Board {
    layers: Vec<Option<Layer>>,
    player: Option<usize>,
    cur_layer: usize,
    delay: usize,
}

impl Board {
    fn new(layers: Vec<Option<Layer>>, delay: usize) -> Board {
        if delay > 0 {
            Board {
                layers: layers,
                player: None,
                cur_layer: 0,
                delay: 0,
            }
        } else {
            Board {
                layers: layers,
                player: Some(0),
                cur_layer: 0,
                delay: 0,
            }
        }
    }

    fn tick_layers(&mut self) {
        for layer in &mut self.layers {
            if let &mut Some(ref mut layer) = layer {
                layer.tick();
            }
        }
    }

    fn run_to_end(&mut self) -> usize {
        let mut severities = vec![];
        let mut i = 0;
        loop {
            self.cur_layer = i;
            //println!("{:?}", self);
            //println!("player is at {}", self.player);
            //println!("checking {}", i);
            if let Some(ref layer) = self.layers[i] {
                //println!("Found layer {} scanner is at position {}", i, layer.cur);
                if self.player.is_some() && i == self.player.unwrap() && layer.at_top() {
                    //println!("adding severity {} * {} = {}", i, layer.depth, i * layer.depth);
                    severities.push(i * layer.depth);
                }
            }
            match self.player.as_mut() {
                Some(p) => *p += 1,
                None => {
                }
            };
            self.tick_layers();
            i += 1;
            if self.is_at_end() {
                break
            }
        }
        severities.iter().sum()
    }

    fn is_at_end(&self) -> bool {
        match self.player {
            Some(ref player) => *player == self.layers.len(),
            None => false,
        }
    }

    fn number_row(&self) -> String {
        (0..self.layers.len()).map(|s| s.to_string()).collect::<Vec<_>>().join("   ")
    }

    fn max_depth(&self) -> usize {
        let mut max = 0;
        for layer in &self.layers {
            if let &Some(ref layer) = layer {
                if layer.depth > max {
                    max = layer.depth;
                }
            }
        }
        max
    }

    fn top_layer_row(&self) -> String {
        let mut cells = vec![];
        for i in 0..self.layers.len() {
            cells.push(self.repr_cell(i, 0, Some("...".into())));
        }
        cells.join(" ")
    }

    fn other_row(&self, row: usize) -> String {
        let mut cells = vec![];
        for i in 0..self.layers.len() {
            cells.push(self.repr_cell(i, row, None));
        }
        cells.join(" ")
    }

    fn repr_cell(&self, idx: usize, row: usize, empty_fill: Option<String>) -> String {
        if let &Some(ref layer) = &self.layers[idx] {
            if row < layer.depth {
                let fill_char = if row == (layer.cur - 1) {
                    "S"
                } else {
                    " "
                };
                if idx == self.cur_layer && row == 0 {
                    format!("({})", fill_char)
                } else {
                    format!("[{}]", fill_char)
                }
            } else {
                "   ".into()
            }
        } else {
            if idx == self.cur_layer && row == 0 {
                "(.)".into()
            } else {
                empty_fill.unwrap_or("   ".into())
            }
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_depth = self.max_depth();
        writeln!(f, " {}", self.number_row())?;
        writeln!(f, "{}", self.top_layer_row())?;
        for row in 1..max_depth {
            writeln!(f, "{}", self.other_row(row));
        }
        Ok(())
    }
}


fn run(input: &str) -> usize {
    let layer_input = input.trim()
                           .lines()
                           .map(|s| s.trim())
                           .filter(|s| !s.is_empty())
                           .map(|s| s.split(": "))
                           .map(|mut spl| (spl.next().expect("didn't get left side").parse::<usize>().expect("Couldn't parse left side"),
                                           spl.next().expect("didn't get right side").parse::<usize>().expect("Couldn't parse right side")))
                           .collect::<Vec<_>>();
    //println!("layer input: {:?}", layer_input);
    let max_idx = layer_input[layer_input.len() - 1].0;
    let mut layers = Vec::with_capacity(max_idx);
    let mut cur_idx = 0;
    for i in 0..max_idx + 1 {
        if layer_input[cur_idx].0 == i {
            layers.push(Some(Layer::new(layer_input[cur_idx].1)));
            cur_idx += 1;
        } else {
            layers.push(None);
        }
    }
    let mut board = Board::new(layers, 0);
    let cost = board.run_to_end();
    cost
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"
            0: 3
            1: 2
            4: 4
            6: 4
        "#;
        assert_eq!(run(&input), 24);
    }

    #[test]
    fn test_answer() {
        let input = r#"
            0: 3
            1: 2
            2: 4
            4: 6
            6: 5
            8: 8
            10: 6
            12: 4
            14: 8
            16: 6
            18: 8
            20: 8
            22: 6
            24: 8
            26: 9
            28: 12
            30: 8
            32: 14
            34: 10
            36: 12
            38: 12
            40: 10
            42: 12
            44: 12
            46: 12
            48: 12
            50: 14
            52: 12
            54: 14
            56: 12
            60: 14
            62: 12
            64: 14
            66: 14
            68: 14
            70: 14
            72: 14
            74: 14
            78: 26
            80: 18
            82: 17
            86: 18
            88: 14
            96: 18
        "#;
        println!("{}", run(&input));
    }
}
