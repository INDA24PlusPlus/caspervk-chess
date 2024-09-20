use caspervk_chess::{BoardState, Game};
use debugutils::board_pos_to_index;
use std::io;
mod debugutils;

pub use debugutils::visualise_board;

fn main() {
    let mut game = Game::new();
    let mut selected_piece: Option<String> = None; // Use an Option to manage state
    let mut board_state: BoardState = BoardState::Default;
    loop {
        print!("{}[2J", 27 as char);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        println!("{:?}", game.board_pieces_sides[5]);
        
        if input == "kill" {
            break;
        }

        if selected_piece.is_none() {
            selected_piece = Some(input.to_string()); // Store input in an Option
            if let Some(ref piece) = selected_piece {
                visualise_board(game.board_pieces, game.board_pieces_sides, game.get_position_possible_movements(board_pos_to_index(piece.clone())));
            }
        } else {
            if let Some(piece) = selected_piece.take() { // Take the value out of the Option
                board_state = game.do_move(board_pos_to_index(piece), board_pos_to_index(input.to_string()));
                visualise_board(game.board_pieces, game.board_pieces_sides, Vec::new());
                println!("\n {:?}", board_state);
                println!("currturn: {:?}", game.curr_turn);
                selected_piece = None; // Reset selected_piece
            }
        }
    }
}
