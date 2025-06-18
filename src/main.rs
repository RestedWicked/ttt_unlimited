use std::{io::stdin, num::ParseIntError};

use bevy::prelude::*;
use ttt_lib::TicTacToe;

#[derive(Component)]
struct Grid(Vec<u8>);

#[derive(Component)]
struct GridRoot(usize);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
    //game()
}

fn ttt_game_old() {
    let mut ttt = TicTacToe::default();
    println!("Welcome to TicTacToe");
    println!("{}", &ttt);

    let mut current_player = 1;
    loop {
        println!("Your move {}: (1,9) ", current_player);
        let mut input_buffer = String::new();
        stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to Read Line");

        let player_move: Result<usize, ParseIntError> = input_buffer.trim().parse();
        if player_move.is_err() {
            println!("Input is not an integer");
            continue;
        }

        let player_move = player_move.unwrap();

        match player_move {
            1..10 => {}
            _ => {
                println!("Input is not between 1-9");
                continue;
            }
        }

        if ttt.state[player_move - 1] != 0 {
            println!("This location is occupied");
            continue;
        }

        ttt.state[player_move - 1] = current_player;

        println!("{}", ttt);

        if ttt.verify_win(current_player) {
            break;
        }

        if current_player == 1 {
            current_player = 2;
        } else {
            current_player = 1;
        }
    }
}
