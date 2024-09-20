use caspervk_chess::Piece;
use caspervk_chess::Side;
use colored::Colorize;
pub fn get_piece_visualisation_char(piece: Piece, side: Side) -> char{
    match piece{
        Piece::Pawn => {
            if(side == Side::Black)
            {
                return '♙';
            } 
            else{
                return '♟';
            }
            },
        Piece::Rook => {
            if(side == Side::Black)
            {
                return '♖';
            } 
            else{
                return '♜';
            }
            },
        Piece::Knight => {
            if(side == Side::Black)
            {
                return '♘';
            } 
            else{
                return '♞';
            }
            },
        Piece::Queen => {
            if(side == Side::Black)
            {
                return '♕';
            } 
            else{
                return '♛';
            }
            },
        Piece::King => {
            if(side == Side::Black)
            {
                return '♔';
            } 
            else{
                return '♚';
            }
            },
        Piece::Bishop => {
            if(side == Side::Black)
            {
                return '♗';
            } 
            else{
                return '♝';
            }
            },
        _ => return ('0')
    }
}

pub fn visualise_board(board: [Piece; 64], boardSides: [Side; 64], selectedPieces: Vec<i8>){
    for i in 1..board.len()+1 {
        if(selectedPieces.contains(&((i-1) as i8))){
            print!("{}  ", get_piece_visualisation_char(board[i-1], boardSides[i-1]).to_string().yellow());
        }
        else{
            print!("{}  ", get_piece_visualisation_char(board[i-1], boardSides[i-1]));
        }
        if i % 8 == 0{
            print!("\n");
        }
    }
}
