use std::{convert::TryFrom, default, ops::Not, result};

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

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
            Side::None => Side::None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoardState {
    Default,
    Checked(Side),
    CheckMated(Side),
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
    pub white_rook_one_moved: bool,
    pub white_rook_two_moved: bool,
    pub black_rook_one_moved: bool,
    pub black_rook_two_moved: bool,
    pub white_king_moved: bool,
    pub black_king_moved: bool
}

const INITIAL_BOARD_PIECES: [Piece; 64] = [
    Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
    Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn,
    Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn, Piece::Pawn,
    Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
];

const INITIAL_BOARD_SIDES: [Side; 64] = [
    Side::White, Side::White, Side::White, Side::White, Side::White, Side::White, Side::White, Side::White,
    Side::White, Side::White, Side::White, Side::White, Side::White, Side::White, Side::White, Side::White,
    Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None,
    Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None,
    Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None,
    Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None, Side::None,
    Side::Black, Side::Black, Side::Black, Side::Black, Side::Black, Side::Black, Side::Black, Side::Black,
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
    history_board_pieces: Vec<[Piece; 64]>,
    history_board_pieces_sides: Vec<[Side; 64]>,
}

impl Game{
    pub fn new() -> Self {
        return Self{
            board_pieces: INITIAL_BOARD_PIECES,
            board_pieces_sides: INITIAL_BOARD_SIDES,
            curr_turn: Side::White,
            castle_info: CastleInfo
            {
                black_king_moved: false, 
                white_king_moved: false, 
                white_rook_one_moved: false, 
                white_rook_two_moved: false, 
                black_rook_one_moved: false, 
                black_rook_two_moved: false
            },
            fifty_move_rule: 50,
            white_king_pos: 4,
            black_king_pos: 60,
            last_move_origin: -1,
            last_move_target: -1,
            pawn_awaiting_promotion_pos: -1,
            history_board_pieces: Vec::new(),
            history_board_pieces_sides: Vec::new(),
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
        if !self.castle_info.white_king_moved {
            if !self.castle_info.white_rook_one_moved && self.can_castle(&[2, 3]) && self.board_pieces[1] == Piece::None {
                out.push(2);
            } else if !self.castle_info.white_rook_two_moved && self.can_castle(&[5, 6]) {
                out.push(6);
            }
        }
    }

    fn add_black_castling_moves(&self, out: &mut Vec<i8>) {
        if !self.castle_info.black_king_moved {
            if !self.castle_info.black_rook_one_moved && self.can_castle(&[58, 59]) && self.board_pieces[57] == Piece::None {
                out.push(58);
            } else if !self.castle_info.black_rook_two_moved && self.can_castle(&[61, 62]) {
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
            if position >= 17{ self.push_pos_if_non_ally(position-17, side, out); }
            if position <= 46{ self.push_pos_if_non_ally(position+17, side, out); }
        }

        if(!is_pos_on_right_edge(position)){
            if position >= 15{ self.push_pos_if_non_ally(position-17, side, out); }
            if position <= 48{ self.push_pos_if_non_ally(position+15, side, out); }
        }

        if position >= 10 && position % 8 >= 2 { self.push_pos_if_non_ally(position-10, side, out); }
        if position <= 53 && position % 8 <= 6 { self.push_pos_if_non_ally(position+10, side, out); }

        if position >= 6 && position % 8 <= 6 { self.push_pos_if_non_ally(position-6, side, out); }
        if position <= 57 && position % 8 >= 2 { self.push_pos_if_non_ally(position+6, side, out); }
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

    fn filter_out_moves_causing_self_check(&self, origin: i8, to_filter: Vec<i8>) -> Vec<i8>{
        let mut to_return = Vec::new();
        for target in to_filter {
            let mut cloned_game = self.clone();
            cloned_game.do_move_internal(origin, target, true);

            let was_checked = cloned_game.is_checked(cloned_game.get_curr_turn_king_pos(), false);

            if !was_checked {
                to_return.push(target);
            }
        }
        return to_return;
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
        return self.filter_out_moves_causing_self_check(position, to_return);
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
        let king_pos = self.get_curr_turn_king_pos();
        if Self::is_checked(&self, king_pos, false){
            if Self::is_checked_mate(&self, king_pos){
                return BoardState::CheckMated(self.curr_turn);
            }
            return BoardState::Checked(self.curr_turn);
        }
        return BoardState::Default;
    }

    fn rank_contains_pawn(to_check: Vec<Piece>) -> bool{
        return to_check.iter().any(|&a| a==Piece::Pawn);
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
        if(Self::rank_contains_pawn(self.board_pieces[0..7].to_vec())){
            return BoardState::BlackPromotion;
        }
        else if(Self::rank_contains_pawn(self.board_pieces[56..63].to_vec())){
            return BoardState::WhitePromotion;
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

    fn get_side_castling_move(king_start: i8, origin: i8, target: i8, rook_one_moved: bool, rook_two_moved: bool, king_moved: bool) -> Option<[i8; 2]>{
        if(origin == king_start && !king_moved){
            if(target == king_start-2 && !rook_one_moved){
                return Some([king_start-4, king_start-1]);
            }
            else if(target == king_start+2 && !rook_two_moved){
                return Some([king_start+3, king_start-1]);
            }
        }    
        return None;
    }

    //returns a move containing the origin of the pawn that is being en passanted and the target of the same pawn. Will cause a deletion of that pawn.
    fn get_en_passant_move(&self, origin: i8, target: i8) -> Option<[i8; 2]>{
        if(self.curr_turn == Side::White && target > 7 && self.board_pieces[(target-8) as usize] == Piece::Pawn && self.last_move_origin == target+8 && self.last_move_target == target-8){
            return Some([target-8, target-8]);
        }
        else if(target < 56 && self.board_pieces[(target+8) as usize] == Piece::Pawn && self.last_move_origin == target-8 && self.last_move_target == target+8){
            return Some([target+8, target+8]);
        }
        return None;
    }
    
    fn update_pieces_has_moved_status(&mut self, origin: i8){
        match origin {
            0 => { self.castle_info.white_rook_one_moved = true;},
            7 => { self.castle_info.white_rook_two_moved = true;},
            56 => { self.castle_info.black_rook_one_moved = true;},
            63 => { self.castle_info.black_rook_two_moved = true;},
            4 => { self.castle_info.white_king_moved = true;},
            60 => { self.castle_info.black_king_moved = true;},
            _ => {}
        }
    }
    // "on_clone" refers to the method being called when the object is being cloned to check for possible movements causing a self check. We dont want to do certain things if that is the case as it will cause stack overflow.
    fn do_move_internal(&mut self, origin: i8, target: i8, on_clone: bool) -> BoardState{
        let mut moves_to_perform = Vec::new();

        moves_to_perform.push(Some([origin, target]));
        moves_to_perform.push(self.get_en_passant_move(origin, target));
        if(self.curr_turn == Side::White){
            moves_to_perform.push(Self::get_side_castling_move(4, origin, target, self.castle_info.white_rook_one_moved, self.castle_info.white_rook_two_moved, self.castle_info.white_king_moved));  
        }
        else{
            moves_to_perform.push(Self::get_side_castling_move(60, origin, target, self.castle_info.black_rook_one_moved, self.castle_info.black_rook_two_moved, self.castle_info.black_king_moved));
        }

        for m in moves_to_perform.iter(){
            if !m.is_none(){
                let value = m.unwrap();
                let value_origin = value[0] as usize;
                let value_target = value[1] as usize;
                self.board_pieces[value_target] = self.board_pieces[value_origin];
                self.board_pieces_sides[value_target] = self.board_pieces_sides[value_origin];
                self.board_pieces[value_origin] = Piece::None;
                self.board_pieces_sides[value_origin] = Side::None
            }
        }

        if !on_clone{
            if self.should_reset_fifty_move_rule(origin, target){
                self.fifty_move_rule = 50;
            }
            else{
                self.fifty_move_rule -= 1;
            }
            return self.get_board_state();
        }
        return BoardState::Default;
    }
    
    pub fn do_move(&mut self, origin: i8, target: i8) -> BoardState {
        self.history_board_pieces.push(self.board_pieces);
        self.history_board_pieces_sides.push(self.board_pieces_sides);
        let toReturn = self.do_move_internal(origin, target, false);
        self.update_pieces_has_moved_status(origin);
        self.last_move_origin = origin;
        self.last_move_target = target;
        self.curr_turn = !self.curr_turn;
        return toReturn;
    }

    pub fn request_draw(&self) -> bool{
        let mut repeated_positions_count = 0;
        for i in 0..self.history_board_pieces.len(){
            for j in 0..i{
                if(self.history_board_pieces[i] == self.history_board_pieces[j] && self.history_board_pieces_sides[i] == self.history_board_pieces_sides[j]){
                    repeated_positions_count += 1;
                }
            }
        }
        return repeated_positions_count > 2;
    }
}
