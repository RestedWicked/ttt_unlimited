use core::fmt;
use std::ops::Index;

pub struct TicTacToe {
    pub state: Vec<u8>,
    pub grid_root: usize,
}

impl TicTacToe {
    pub fn len(self) -> usize {
        self.state.len()
    }

    pub fn custom_size(size: usize) -> Self {
        let length = size * size;

        TicTacToe {
            state: vec![0; length],
            grid_root: size,
        }
    }

    pub fn row(&self, r: usize) -> &[u8] {
        let start = r * self.grid_root;
        let end = (r + 1) * self.grid_root;
        &self.state[start..end]
    }

    pub fn col(&self, c: usize) -> Vec<u8> {
        let mut val: Vec<u8> = Vec::new();
        for i in 0..self.grid_root {
            val.push(self[(c, i)]);
        }
        val
    }

    pub fn diag_down(&self) -> Vec<u8> {
        let mut val: Vec<u8> = Vec::new();
        for i in 0..self.grid_root {
            val.push(self[(i, i)]);
        }
        val
    }

    pub fn diag_up(&self) -> Vec<u8> {
        let mut val: Vec<u8> = Vec::new();
        for (i, j) in (0..self.grid_root).zip((0..self.grid_root).rev()) {
            val.push(self[(i, j)]);
        }
        val
    }

    pub fn verify_win(&self, player: u8) -> bool {
        let win_state = vec![player; self.grid_root];

        for i in 0..self.grid_root {
            if win_state == self.row(i) {
                return true;
            }

            if win_state == self.col(i) {
                return true;
            }
        }

        if win_state == self.diag_down() {
            return true;
        }

        if win_state == self.diag_up() {
            return true;
        }

        false
    }
}
impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe {
            state: vec![0; 9],
            grid_root: 3,
        }
    }
}

impl Index<(usize, usize)> for TicTacToe {
    type Output = u8;
    fn index(&self, xy: (usize, usize)) -> &Self::Output {
        let (x, y) = xy;
        assert!(x < self.grid_root, "Index X was out of scope");
        assert!(y < self.grid_root, "Index Y was out of scope");
        let i = x + (y * self.grid_root);

        &self.state[i]
    }
}

impl Index<usize> for TicTacToe {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self.state[i]
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = &self.state;
        let side = state.len().isqrt();

        for i in 0..state.len() {
            let mut sign = ".";

            if state[i] == 2 {
                sign = "O";
            }
            if state[i] == 1 {
                sign = "X";
            }

            if (i + 1) % side == 0 {
                writeln!(f, "{}", sign)?
            } else {
                write!(f, "{}", sign)?
            }
        }

        Ok(())
    }
}
