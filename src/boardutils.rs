#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece{
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Side{
    White = 0,
    Black = 1,
    None = 2
}

#[derive(Debug, Clone)]
pub enum BoardState{
    Default = 0,
    WhiteChecked = 1,
    BlackChecked = 2,
    WhiteLoseByCheckMate = 3,
    BlackLoseByCheckMate = 4,
    WhiteLoseByTime = 5,
    BlackLoseByTime = 6,
    DrawBy50Rule = 7,
    DrawByStaleMate = 8,
    WhitePromotion = 9,
    BlackPromotion = 10,
}
#[derive(Debug, Clone)]
pub struct CastleInfo{
    pub top_right_rook_moved: bool,
    pub bottom_right_rook_moved: bool,
    pub top_left_rook_moved: bool,
    pub bottom_left_rook_moved: bool,
    pub top_king_moved: bool,
    pub bottom_king_moved: bool
}

pub const INITIAL_BOARD_PIECES: [Piece; 64] = [
    // First rank (White's major pieces)
    Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
    // Second rank (White's pawns)
    Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn,
    // Middle ranks (empty squares)
    Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
    // Seventh rank (Black's pawns)
    Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn,
    // Eighth rank (Black's major pieces)
    Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
];

pub const INITIAL_BOARD_SIDES: [Side; 64] = [
    // First rank (White's major pieces)
    Side::White, Side::White, Side::White, Side::White, Side::White, Side::White, Side::White, Side::White,
    // Second rank (White's pawns)
    Side::White, Side::White, Side::White, Side::White, Side::White, Side::White, Side::White, Side::White,
    // Middle ranks (empty squares)
    Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None,
    Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None,
    Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None,
    Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None,
    // Seventh rank (Black's pawns)
    Side::Black, Side::Black, Side::Black, Side::Black, Side::Black, Side::Black, Side::Black, Side::Black,
    // Eighth rank (Black's major pieces)
    Side::Black, Side::Black, Side::Black, Side::Black, Side::Black, Side::Black,Side::Black, Side::Black
];

pub fn is_pos_outside_of_board(position: i8) -> bool{
    return position > 63 || position < 0;
}
pub fn is_pos_on_right_edge(position: i8) -> bool{
    return (position+1)%8 == 0;
}
pub fn is_pos_on_left_edge(position: i8) -> bool{
    return position%8 == 0;
}

pub type DirCallback = fn(i8, i8) -> (i8, bool);

pub fn right_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos+n;
    return (target, is_pos_on_right_edge(target-1) && n != 0);
}
pub fn left_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos-n;
    return (target, is_pos_on_left_edge(target+1) && n != 0);
}
pub fn up_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos-n*8;
    return (target, is_pos_outside_of_board(target));
}
pub fn down_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos+n*8;
    return (target, is_pos_outside_of_board(target));
}


pub fn top_right_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos - n*9;
    return (target, is_pos_on_right_edge(target))
}

pub fn top_left_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos - n*7;
    return (target, is_pos_on_left_edge(target))
}

pub fn bottom_right_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos + n*7;
    return (target, is_pos_on_right_edge(target))
}

pub fn bottom_left_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos + n*9;
    return (target, is_pos_on_left_edge(target))
}
