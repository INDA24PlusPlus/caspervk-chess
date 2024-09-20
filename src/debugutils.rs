use caspervk_chess::Piece;
use caspervk_chess::Side;
use colored::Colorize;

pub fn board_pos_to_index(board_pos: String) -> i8{
    let mut toReturn = 0;
    toReturn += (board_pos.chars().nth(1).unwrap().to_digit(10).unwrap()-1)*8;
    match board_pos.to_lowercase().chars().nth(0).unwrap(){
        'a' => toReturn += 0,
        'b' => toReturn += 1,
        'c' => toReturn += 2,
        'd' => toReturn += 3,
        'e' => toReturn += 4,
        'f' => toReturn += 5,
        'g' => toReturn += 6,
        'h' => toReturn += 7,
        _ => unimplemented!(),
    }
    return toReturn as i8;
}
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
