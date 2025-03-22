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
        state: vec![0,0,1,0,1,0,1,0,0]
    };

    verify_win_condition(&ttt, 1);

    println!("{}", ttt);
}

fn game() {
    todo!()
}



// Win Conditions for a 3x3 Grid
//
// 1 0 0 1 0 0 1 0 0 Vertical Left
// 0 1 0 0 1 0 0 1 0 Vertical Center
// 0 0 1 0 0 1 0 0 1 Vertical Right
//
// 1 1 1 0 0 0 0 0 0 Horizontal Top
// 0 0 0 1 1 1 0 0 0 Horizontal Center
// 0 0 0 0 0 0 1 1 1 Horizontal Bottom
//
// 1 0 0 0 1 0 0 0 1 Diagonal Downward
// 1 0 0 0 - 0 1 0 0 - 0 0 1 0 - 0 0 0 1
// 0 0 1 0 1 0 1 0 0 Diagonal Upward
// 0 0 0 1 - 0 0 1 0 - 0 1 0 0 - 1 0 0 0
fn verify_win_condition(game: &TicTacToe, player: u8) {
    // The TicTacToe grid is represented as an array of N length.
    // If the square root of N is an integer we have a valid grid.
    // Here I refer to it as the Grid Root.
    let grid_root = game.state.len().isqrt();

    // In the TicTacToe grid Players are represented as the values
    // 1 and 2, to simplify the algorithm, I check one player at a time.
    // The mask simply removes the other player from the grid.
    let mut mask = vec![0; game.state.len()];
    for (i, val) in game.state.iter().enumerate() {
        if *val == player {
            mask[i] = 1;
        }
    }
    
    let win_state = vec![1; grid_root];

    let mut d_mask_down = vec![0; grid_root];
    let mut d_mask_up = vec![0; grid_root];

    // This Loop handles verifying a win state in the tic tac toe grid.
    for i in 0..grid_root {

        // The min and max values are used to get a row from the grid
        // and check if it meets the win condition, each iteration of 
        // loop checks the next row.
        let min: usize = i * grid_root;
        let max: usize = (i + 1) * grid_root;
        if mask[min..max] == win_state {
            println!("You win Horizontal");
        }
        
        // This one liner gets me each column to check if it meets the win
        // condition.
        let v_mask: Vec<_> = mask.iter().skip(i).step_by(grid_root).copied().collect();
        if v_mask == win_state {
            println!("You win Vertical");
        }

        d_mask_down[i] = mask[i * (grid_root + 1)];
        d_mask_up[i] = game.state[(i + 1) * (grid_root - 1)];

    }
    
    if d_mask_down == win_state || d_mask_up == win_state {
        println!("You win Diagonal");
    }
}
