
use std::io;

use crate::game::{Game};

mod game;

fn main() {
    println!("Game!");

    let mut board = game::Board::new();

    loop {
        let mut pos;

        loop {
            println!("Players turn: {}. Type a number from 1 to 9", board.current_player());
            // read a pos
            pos = get_input();

            if let Err(e) = board.play(pos) {
                println!("error, play again: {}. pos: {}", e.message, e.position);
            } else {
                break;
            }
        }

        board.print_board();

        let state = board.check_game_state();

        match state {
            game::GameState::DRAW => { println!("Draw!"); break; }
            game::GameState::OVICTORY => { println!("Victory of player O!"); break; }
            game::GameState::XVICTORY => { println!("Victory of player X!"); break; }
            _ => {
                board.switch_players();
            }
        }
        
    }

    // or, implement the Display and Debug trait to our error and just print it
}

fn get_input() -> i32 {
    let mut input = String::new();

    io::stdin().
        read_line(&mut input)
        .expect("Failed to read Line");

    let n : i32 = input
                    .trim()
                    .parse()
                    .expect("invalid number");

    return n;
}