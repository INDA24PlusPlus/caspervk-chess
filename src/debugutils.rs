use caspervk_chess::Piece;
use caspervk_chess::Side;

pub fn get_piece_visualisation_char(piece: Piece) -> char{
    match piece{
        Piece::Pawn => return 'p',
        Piece::Rook => return 'r',
        Piece::Knight => return 'n',
        Piece::Queen => return 'q',
        Piece::King => return 'k',
        Piece::Bishop => return 'b',
        _ => return ('0')
    }
}

pub fn visualise_board(board: [Piece; 64], boardSides: [Side; 64]){
    let mut to_print: String = String::new();
    for i in 1..board.len()+1 {
        to_print.push(get_piece_visualisation_char(board[i-1]));
        if(boardSides[i-1]==Side::Black){
            to_print.push('b');
        }
        else if boardSides[i-1]==Side::White{
            to_print.push('w');
        }
        else{
            to_print.push('0')
        }
            to_print.push_str("  ");
        if i % 8 == 0{
            to_print.push_str("\n\n");
        }
    }
    println!("{}", to_print);
}
