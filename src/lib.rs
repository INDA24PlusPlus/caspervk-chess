use std::{default, result};

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
pub enum Side {
    White,
    Black,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoardState {
    Default,
    Check(Side),
    CheckMate(Side),
    WhiteLoseByCheckMate,
    BlackLoseByCheckMate,
    WhiteLoseByTime,
    BlackLoseByTime,
    DrawBy50Rule,
    DrawByStaleMate,
    WhitePromotion,
    BlackPromotion,
}
#[derive(Debug, Clone)]
struct CastleInfo{
    pub top_right_rook_moved: bool,
    pub bottom_right_rook_moved: bool,
    pub top_left_rook_moved: bool,
    pub bottom_left_rook_moved: bool,
    pub top_king_moved: bool,
    pub bottom_king_moved: bool
}

const INITIAL_BOARD_PIECES: [Piece; 64] = [
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

const INITIAL_BOARD_SIDES: [Side; 64] = [
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

fn is_pos_outside_of_board(position: i8) -> bool{
    return position > 63 || position < 0;
}
fn is_pos_on_right_edge(position: i8) -> bool{
    return (position+1)%8 == 0;
}
fn is_pos_on_left_edge(position: i8) -> bool{
    return position%8 == 0;
}

type DirCallback = fn(i8, i8) -> (i8, bool);

fn right_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos+n;
    return (target, is_pos_on_right_edge(target-1) && n != 0);
}
fn left_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos-n;
    return (target, is_pos_on_left_edge(target+1) && n != 0);
}
fn up_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos-n*8;
    return (target, is_pos_outside_of_board(target));
}
fn down_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos+n*8;
    return (target, is_pos_outside_of_board(target));
}


fn top_right_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos - n*9;
    return (target, is_pos_on_right_edge(target))
}
fn top_left_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos - n*7;
    return (target, is_pos_on_left_edge(target))
}
fn bottom_right_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos + n*7;
    return (target, is_pos_on_right_edge(target))
}
fn bottom_left_callback(original_pos: i8, n: i8) -> (i8, bool){
    let target = original_pos + n*9;
    return (target, is_pos_on_left_edge(target))
}

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

#[derive(Clone)]
pub struct Game{
    pub board_pieces: [Piece; 64],
    pub board_pieces_sides: [Side; 64],
    pub curr_turn: Side,
    castle_info: CastleInfo,
    fifty_move_rule: i8,
    white_king_pos: i8,
    black_king_pos: i8,
    pub last_move_origin: i8,
    pub last_move_target: i8,
    pawn_awaiting_promotion_pos: i8,
}

impl Game{
    pub fn new() -> Self {
        return Self{
            board_pieces: INITIAL_BOARD_PIECES,
            board_pieces_sides: INITIAL_BOARD_SIDES,
            curr_turn: Side::White,
            castle_info: CastleInfo
            {
                top_king_moved: false, 
                bottom_king_moved: false, 
                bottom_right_rook_moved: false, 
                bottom_left_rook_moved: false, 
                top_left_rook_moved: false, 
                top_right_rook_moved: false
            },
            fifty_move_rule: 50,
            white_king_pos: 4,
            black_king_pos: 60,
            last_move_origin: -1,
            last_move_target: -1,
            pawn_awaiting_promotion_pos: -1
        };
    }

    fn has_pawn_moved(position: i8, side: Side) -> bool{
        if(side == Side::White){
            return position < 8 || position > 17;
        }
        else{
            return position < 47 || position > 56;
        }
    }

    //returns true if the position is blocked
    fn push_pos_if_non_ally(&self, position: i8, side: Side, out: &mut Vec<i8>) -> bool{
        let target_pos_side = self.board_pieces_sides[position as usize];
        if target_pos_side == side{ 
            return true;
        }
        out.push(position);
        if target_pos_side == Side::None{ 
            return false;
        }
        return true;
    }

    //Takes in callbacks describing the directions and returns possible movements the piece can make. Useful for king, queen, bishop and rook.
    fn get_directions_movements(&self, movements: &mut Vec<Box<dyn Fn(i8, i8) -> (i8, bool)>>,  position: i8, side: Side, out: &mut Vec<i8>, allowedLength: u8){
        for n in 0..allowedLength{
            for c in (0..movements.len()).rev() {
                let (target, pos_invalid) = movements[c](position, n as i8);
                if is_pos_outside_of_board(target) || pos_invalid || (n != 0 && Self::push_pos_if_non_ally(&self, target, side, out)){
                    movements.remove(c);
                }
            }
        }
    }

    fn get_piece_orthogonal_movements(&self, position: i8, side: Side, out: &mut Vec<i8>, allowedLength: u8){
        let mut orthogonal_callbacks: Vec<Box<dyn Fn(i8, i8) -> (i8, bool)>> = Vec::new();
        orthogonal_callbacks.push(Box::new(right_callback));
        orthogonal_callbacks.push(Box::new(left_callback));
        orthogonal_callbacks.push(Box::new(up_callback));
        orthogonal_callbacks.push(Box::new(down_callback));
        Self::get_directions_movements(&self, &mut orthogonal_callbacks, position, side, out, allowedLength);
    }

    fn get_piece_diagonal_movements(&self, position: i8, side: Side, out: &mut Vec<i8>, allowedLength: u8){
        let mut diagonal_callbacks: Vec<Box<dyn Fn(i8, i8) -> (i8, bool)>> = Vec::new();
        diagonal_callbacks.push(Box::new(top_right_callback));
        diagonal_callbacks.push(Box::new(top_left_callback));
        diagonal_callbacks.push(Box::new(bottom_right_callback));
        diagonal_callbacks.push(Box::new(bottom_left_callback));
        Self::get_directions_movements(&self, &mut diagonal_callbacks, position, side, out, allowedLength);
    }

    fn get_rook_possible_movements(&self, position: i8, side: Side, out: &mut Vec<i8>){
        Self::get_piece_orthogonal_movements(&self, position, side, out, 8);
    }

    fn add_castling_moves(&self, out: &mut Vec<i8>) {
        if !self.is_checked(self.get_curr_turn_king_pos(), false) {
            match self.curr_turn {
                Side::White => self.add_white_castling_moves(out),
                Side::Black => self.add_black_castling_moves(out),
                _ => {},
            }
        }
    }

    fn add_white_castling_moves(&self, out: &mut Vec<i8>) {
        if !self.castle_info.top_king_moved {
            if !self.castle_info.top_left_rook_moved && self.can_castle(&[1, 2, 3]) {
                out.push(2);
            } else if !self.castle_info.top_right_rook_moved && self.can_castle(&[5, 6]) {
                out.push(6);
            }
        }
    }

    fn add_black_castling_moves(&self, out: &mut Vec<i8>) {
        if !self.castle_info.bottom_king_moved {
            if !self.castle_info.bottom_left_rook_moved && self.can_castle(&[57, 58, 59]) {
                out.push(58);
            } else if !self.castle_info.bottom_right_rook_moved && self.can_castle(&[61, 62]) {
                out.push(62);
            }
        }
    }

    fn get_king_possible_movements(&self, position: i8, side: Side, out: &mut Vec<i8>, filter: bool){
        Self::get_piece_orthogonal_movements(&self, position, side, out, 2);
        Self::get_piece_diagonal_movements(&self, position, side, out, 2);
        if(filter){
            self.add_castling_moves(out);
        }
    }
    
    fn get_queen_possible_movements(&self, position: i8, side: Side, out: &mut Vec<i8>){
        Self::get_piece_orthogonal_movements(&self, position, side, out, 8);
        Self::get_piece_diagonal_movements(&self, position, side, out, 8);
    }

    fn get_knight_possible_movements(&self, position: i8, side: Side, out: &mut Vec<i8>){
        if(!is_pos_on_left_edge(position)){
            if position >= 17 && self.board_pieces_sides[(position - 17) as usize] != side {
                out.push(position - 17);
            }
            if position <= 46 && self.board_pieces_sides[(position + 17) as usize] != side {
                out.push(position + 17);
            }
        }

        if(!is_pos_on_right_edge(position)){
            if position >= 15 && self.board_pieces_sides[(position - 15) as usize] != side {
                out.push(position - 15);
            }
            if position <= 48 && self.board_pieces_sides[(position + 15) as usize] != side {
                out.push(position + 15);
            }
        }

        if position >= 10 && position % 8 >= 2 {
            if self.board_pieces_sides[(position - 10) as usize] != side {
                out.push(position - 10);
            }
        }
        if position <= 53 && position % 8 <= 6 {
            if self.board_pieces_sides[(position + 10) as usize] != side {
                out.push(position + 10);
            }
        }

        if position >= 6 && position % 8 <= 6 {
            if self.board_pieces_sides[(position - 6) as usize] != side {
                out.push(position - 6);
            }
        }
        if position <= 57 && position % 8 >= 2 {
            if self.board_pieces_sides[(position + 6) as usize] != side {
                out.push(position + 6);
            }
        }

    }

    fn get_bishop_possible_movements(&self, position: i8, side: Side, out: &mut Vec<i8>){
        Self::get_piece_diagonal_movements(&self, position, side, out, 8);
    }

    fn get_pawn_possible_movements(&self, position: i8, side: Side, out: &mut Vec<i8>){
        let has_pawn_moved = Self::has_pawn_moved(position, side);

        if side == Side::White{
            if(position < 56){
                if self.board_pieces[(position+8) as usize] == Piece::None{
                    out.push(position+8);
                    if !has_pawn_moved && self.board_pieces[(position+16) as usize] == Piece::None{
                        out.push(position+16);
                    }
                }
                if !is_pos_on_left_edge(position) && self.board_pieces_sides[(position+7) as usize] == Side::Black{
                    out.push(position+7);
                }
                if !is_pos_on_right_edge(position) && position != 55 && self.board_pieces_sides[(position+9) as usize] == Side::Black{
                    out.push(position+9);
                }
                // en passant
                if position < 47 && self.board_pieces[(position+1) as usize] == Piece::Pawn && self.last_move_origin == position+17{
                    out.push(position+9);
                }
                if position < 45 && self.board_pieces[(position-1) as usize] == Piece::Pawn && self.last_move_origin == position+15{
                    out.push(position+7);
                }
            }
        }
        else if position > 7{
            if self.board_pieces[(position-8) as usize] == Piece::None{
                out.push(position-8);
                if !has_pawn_moved && self.board_pieces[(position-16) as usize] == Piece::None{
                    out.push(position-16);
                }
            }
            if !is_pos_on_right_edge(position) && self.board_pieces_sides[(position-7) as usize] == Side::White{
                out.push(position-7);
            }
            if !is_pos_on_left_edge(position) && position != 8 && self.board_pieces_sides[(position-9) as usize] == Side::White{
                out.push(position-9);
            }
            // en passant
            if position > 14 && self.board_pieces[(position+1) as usize] == Piece::Pawn && self.last_move_origin == position-15{
                out.push(position-7);
            }
            if position > 16 && self.board_pieces[(position-1) as usize] == Piece::Pawn && self.last_move_origin == position-17{
                out.push(position-9);
            }
        }
    }
    fn get_position_possible_movements_internal(&self, position: i8, filter: bool) -> Vec<i8>{
        let pos_side: Side = self.board_pieces_sides[position as usize];
        let pos_piece: Piece = self.board_pieces[position as usize];
        let mut to_return = Vec::new();

        match pos_piece{
            Piece::Pawn => Self::get_pawn_possible_movements(self, position, pos_side, &mut to_return),
            Piece::Rook => Self::get_rook_possible_movements(self, position, pos_side, &mut to_return),
            Piece::Knight => Self::get_knight_possible_movements(self, position, pos_side, &mut to_return),
            Piece::Queen => Self::get_queen_possible_movements(self, position, pos_side, &mut to_return),
            Piece::King => Self::get_king_possible_movements(self, position, pos_side, &mut to_return, filter),
            Piece::Bishop => Self::get_bishop_possible_movements(self, position, pos_side, &mut to_return),
            _ => {},
        }
        if(!filter){
            return to_return;
        }
        let mut to_return_filtered = Vec::new();
        for possible_movement in to_return {
            let mut cloned_game = self.clone();
            cloned_game.do_move_internal(position, possible_movement, true);

            let was_checked = cloned_game.is_checked(cloned_game.get_curr_turn_king_pos(), false);

            if !was_checked {
                to_return_filtered.push(possible_movement);
            }
        }
        return to_return_filtered;
    }

    pub fn get_position_possible_movements(&self, position: i8) -> Vec<i8>{
        return self.get_position_possible_movements_internal(position, true);
    }

    //takes a pos so that we can check non king positions for castling and checkmate.
    fn is_checked(&self, pos: i8, filter: bool) -> bool{
        for (i, piece_side) in self.board_pieces_sides.iter().enumerate(){
            if(*piece_side != self.curr_turn && Self::get_position_possible_movements_internal(self, i as i8, filter).contains(&pos)){
                return true;
            }
        } 
        return false;
    }
    fn get_curr_turn_king_pos(&self) -> i8{
        if(self.curr_turn == Side::Black){
            return self.black_king_pos;
        }
        return self.white_king_pos;
    }

    fn is_checked_mate(&self, pos: i8) -> bool {    
        let mut piece_movements: Vec<(i8, Vec<i8>)> = Vec::new();
        for (i, &piece_side) in self.board_pieces_sides.iter().enumerate() {
            if piece_side == self.curr_turn{
                let possible_movements = self.get_position_possible_movements(i as i8);
                if !possible_movements.is_empty() {
                    piece_movements.push((i as i8, possible_movements));
                }
            }
        }
    
        for (i, possible_movements) in piece_movements {
            for target in possible_movements {
                let mut cloned_game = self.clone();
                cloned_game.do_move_internal(i, target, true);
    
                let was_checked = cloned_game.is_checked(cloned_game.get_curr_turn_king_pos(), false);
    
                if !was_checked {
                    return false;
                }
            }
        }
    
        return true;
    }

    //gets current turns king threat status. (if checked or checkmated)
    fn get_king_threat_status(&self) -> BoardState{
        let king_pos = if self.curr_turn == Side::White {
            self.white_king_pos
        } else {
            self.black_king_pos
        };
        if Self::is_checked(&self, king_pos, false){
            if Self::is_checked_mate(&self, king_pos){
                return BoardState::CheckMate(self.curr_turn);
            }
            return BoardState::Check(self.curr_turn);
        }
        return BoardState::Default;
    }
    
    fn get_board_state(&mut self) -> BoardState{
        let threat_status = Self::get_king_threat_status(&self);
        if threat_status != BoardState::Default{
            return threat_status;
        }
        if self.fifty_move_rule == 0{
            return BoardState::DrawBy50Rule;
        }
        if Self::is_stalemate(&self){
            return BoardState::DrawByStaleMate;
        }
        return BoardState::Default;
    }
    pub fn choose_promotion_piece(&mut self, piece: Piece) -> BoardState{
        self.board_pieces[self.pawn_awaiting_promotion_pos as usize] = piece;
        self.board_pieces_sides[self.pawn_awaiting_promotion_pos as usize] = self.curr_turn;
        return self.get_board_state();
    }

    fn can_castle(&self, path: &[i8]) -> bool {
        path.iter().all(|&p| self.board_pieces[p as usize] == Piece::None && !self.is_checked(p, false))
    }
    
    fn should_reset_fifty_move_rule(&self, move_origin: i8, move_target: i8) -> bool{
        return self.board_pieces[move_target as usize] != Piece::None || self.board_pieces[move_origin as usize] == Piece::Pawn;
    }
    fn is_stalemate(&self) -> bool{
        let mut no_possible_movements = true;
        for (i, &Side) in self.board_pieces_sides.iter().enumerate(){
            if(Side != self.curr_turn && Self::get_position_possible_movements(&self, i as i8).len() > 0){
                no_possible_movements = false;
                break;
            }
        }
        if(no_possible_movements){
            return true;
        }
        return false;
    }
    //this function is so ugly and repetitive but i cba because if split into other function all hell breaks loose with rust compiler
    fn do_move_internal(&mut self, origin: i8, target: i8, on_clone: bool) -> BoardState{
        let mut pawn_awaiting_promo = false;

        //as mentioned above........
        if(self.curr_turn == Side::White){
            if self.board_pieces[origin as usize]==Piece::Pawn{
                if target > 7 && self.board_pieces[(target-8) as usize] == Piece::Pawn && self.last_move_origin == target+8 && self.last_move_target == target-8{
                    //en passant logic
                    self.board_pieces[(target-8) as usize] = Piece::None;
                    self.board_pieces_sides[(target-8) as usize] = Side::None;
                }
                else if target < 7{
                    pawn_awaiting_promo = true;
                }
            }
            //Castling logic
            if !self.castle_info.top_king_moved && !self.is_checked(4, false){
                if !self.castle_info.top_left_rook_moved && target == 2 && !self.is_checked(3, false) && !self.is_checked(2, false) && self.board_pieces[1] == Piece::None && self.board_pieces[2] == Piece::None && self.board_pieces[3] == Piece::None{
                    self.board_pieces[3] = self.board_pieces[0];
                    self.board_pieces_sides[3] = self.board_pieces_sides[0];
                    self.board_pieces[0] = Piece::None;
                    self.board_pieces_sides[0] = Side::None;
                } else if !self.castle_info.top_right_rook_moved && target == 6 && !self.is_checked(5, false) && !self.is_checked(6, false) && self.board_pieces[5] == Piece::None && self.board_pieces[6] == Piece::None{
                    self.board_pieces[5] = self.board_pieces[7];
                    self.board_pieces_sides[5] = self.board_pieces_sides[7];
                    self.board_pieces[7] = Piece::None;
                    self.board_pieces_sides[7] = Side::None;
                }
            }
            match origin {
                0 => { self.castle_info.top_left_rook_moved = true;}
                7 => { self.castle_info.top_right_rook_moved = true;}
                4 => { self.castle_info.bottom_king_moved = true;},
                _ => {}
            }    
        }
        else{
            if self.board_pieces[origin as usize]==Piece::Pawn{
                if(target < 56 && self.board_pieces[(target+8) as usize] == Piece::Pawn && self.last_move_origin == target-8 && self.last_move_target == target+8){
                    //En passant logic
                    self.board_pieces[(target+8) as usize] = Piece::None;
                    self.board_pieces_sides[(target+8) as usize] = Side::None;
                }
                else if(target > 55){
                    pawn_awaiting_promo = true;
                }
            }
            //Castling logic
            if !self.castle_info.bottom_king_moved && !self.is_checked(60, false){
                if !self.castle_info.bottom_left_rook_moved && target == 58 && !self.is_checked(58, false) && !self.is_checked(59, false) && self.board_pieces[57] == Piece::None && self.board_pieces[58] == Piece::None && self.board_pieces[59] == Piece::None{
                    self.board_pieces[59] = self.board_pieces[56];
                    self.board_pieces_sides[59] = self.board_pieces_sides[56];
                    self.board_pieces[56] = Piece::None;
                    self.board_pieces_sides[56] = Side::None;
                } else if !self.castle_info.bottom_right_rook_moved && target == 62 && !self.is_checked(61, false) && !self.is_checked(62, false) && self.board_pieces[61] == Piece::None && self.board_pieces[62] == Piece::None{
                    self.board_pieces[61] = self.board_pieces[63];
                    self.board_pieces_sides[61] = self.board_pieces_sides[63];
                    self.board_pieces[63] = Piece::None;
                    self.board_pieces_sides[63] = Side::None;
                }
            }
            match origin {
                56 => { self.castle_info.bottom_left_rook_moved = true;}
                63 => { self.castle_info.bottom_right_rook_moved = true;},
                60 => { self.castle_info.bottom_king_moved = true;},
                _ => {}
            }  
        }

        self.board_pieces[target as usize] = self.board_pieces[origin as usize];
        self.board_pieces_sides[target as usize] = self.board_pieces_sides[origin as usize];
        self.board_pieces[origin as usize] = Piece::None;
        self.board_pieces_sides[origin as usize] = Side::None;

        if !on_clone{
            if self.should_reset_fifty_move_rule(origin, target){
                self.fifty_move_rule = 50;
            }
            else{
                self.fifty_move_rule -= 1;
            }

            self.last_move_origin = origin;
            self.last_move_target = target;
            if(pawn_awaiting_promo){
                self.pawn_awaiting_promotion_pos = target;
                if(self.curr_turn == Side::White){
                    self.curr_turn = Side::Black;
                    return BoardState::WhitePromotion;
                }
                self.curr_turn = Side::White;
                return BoardState::BlackPromotion;
            }
            if(self.curr_turn == Side::White){
                self.curr_turn = Side::Black;
            }
            else{
                self.curr_turn = Side::White;
            }
            return self.get_board_state();
        }
        return BoardState::Default;
    }
    
    pub fn do_move(&mut self, origin: i8, target: i8) -> BoardState {
        return self.do_move_internal(origin, target, false);
    }
}
