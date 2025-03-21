use std::fmt;

struct TicTacToe {
    state: Vec<u8>
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe {
            state: vec![0; 9]
        }
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

impl TicTacToe {
    fn custom_size(size: usize) -> Self {
        let size = size * size;
        
        TicTacToe {
            state: vec![0; size]
        }
    }
}


fn main() {
    let ttt = TicTacToe {
        state: vec![1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0]
    };

    verify_win_condition(&ttt, 1);

    println!("{}", ttt);
}

fn game() {
    todo!()
}

fn verify_win_condition(game: &TicTacToe, player: u8) {
    // 1 0 0 1 0 0 1 0 0 Vertical Left
    // 0 1 0 0 1 0 0 1 0 Vertical Center
    // 0 0 1 0 0 1 0 0 1 Vertical Right
    //
    // 1 1 1 0 0 0 0 0 0 Horizontal Top
    // 0 0 0 1 1 1 0 0 0 Horizontal Center
    // 0 0 0 0 0 0 1 1 1 Horizontal Bottom
    //
    // 1 0 0 0 1 0 0 0 1 Diagonal Downward
    // 0 0 1 0 1 0 1 0 0 Diagonal Upward
    
    let root = game.state.len().isqrt();
    let mut mask = vec![0; game.state.len()];

    for (i, val) in game.state.iter().enumerate() {
        if *val == player {
            mask[i] = player;
        }
    }

    for i in 0..root {
        let min: usize = i * root;
        let max: usize = (i + 1) * root;
        // println!("{:?}", &mask[min..max]);
        if mask[min..max] == vec![1; root] {
            println!("You win Horizontal");
        }
    }

}
