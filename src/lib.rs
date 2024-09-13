use std::{default, result};

mod boardutils;

// Use items from `boardutils` module
pub use boardutils::{
    is_pos_outside_of_board,
    is_pos_on_right_edge,
    is_pos_on_left_edge,
    DirCallback,
    right_callback,
    left_callback,
    up_callback,
    down_callback,
    top_right_callback,
    top_left_callback,
    bottom_right_callback,
    bottom_left_callback,
    Piece,
    Side,
    BoardState,
    CastleInfo,
    INITIAL_BOARD_PIECES,
    INITIAL_BOARD_SIDES
};

pub struct Game{
    pub board_pieces: [Piece; 64],
    pub board_pieces_sides: [Side; 64],
    pub curr_turn: Side,
    pub castle_info: CastleInfo,
    pub fifty_move_rule: i8,
    pub orthagonal_callbacks: Vec<Box<dyn Fn(usize, usize) -> (usize, bool)>>,
    pub diagonal_callbacks: Vec<Box<dyn Fn(usize, usize) -> (usize, bool)>>
}


impl Game{
    pub fn new() -> Self {
        let mut orthogonal_callbacks: Vec<Box<dyn Fn(usize, usize) -> (usize, bool)>>  = Vec::new();
        orthogonal_callbacks.push(Box::new(right_callback));
        orthogonal_callbacks.push(Box::new(left_callback));
        orthogonal_callbacks.push(Box::new(up_callback));
        orthogonal_callbacks.push(Box::new(down_callback));

        let mut diagonal_callbacks: Vec<Box<dyn Fn(usize, usize) -> (usize, bool)>> = Vec::new();
        diagonal_callbacks.push(Box::new(top_right_callback));
        diagonal_callbacks.push(Box::new(top_left_callback));
        diagonal_callbacks.push(Box::new(bottom_right_callback));
        diagonal_callbacks.push(Box::new(bottom_left_callback));

        Self{
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
            diagonal_callbacks:  diagonal_callbacks,
            orthagonal_callbacks: orthogonal_callbacks
        }
    }

    fn has_pawn_moved(position: usize, side: Side) -> bool{
        if(side == Side::White){
            return position < 8 || position > 17;
        }
        else{
            return position < 47 || position > 56;
        }
    }

    //returns true if the position is blocked
    fn push_pos_if_non_ally(&self, position: usize, side: Side, out: &mut Vec<usize>) -> bool{
        let target_pos_side = self.board_pieces_sides[position];
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
    fn get_directions_movements(&self, movements: &mut Vec<Box<dyn Fn(usize, usize) -> (usize, bool)>>,  position: usize, side: Side, out: &mut Vec<usize>, allowedLength: u8){
        for n in 0..allowedLength{
            let mut count = 0;
            for c in 0..movements.len(){
                let (target, nextPosInvalid) = movements[c-count](position, n as usize);
                if is_pos_outside_of_board(target) || Self::push_pos_if_non_ally(&self, target, side, out) || nextPosInvalid{
                    movements.remove(c-count);
                    count += 1;
                }
            }
        }
    }

    fn get_piece_orthogonal_movements(&self, position: usize, side: Side, out: &mut Vec<usize>, allowedLength: u8){

        Self::get_directions_movements(&self, &mut self.orthogonal_callbacks, position, side, out, allowedLength);
    }

    fn get_piece_diagonal_movements(&self, position: usize, side: Side, out: &mut Vec<usize>, allowedLength: u8){
        Self::get_directions_movements(&self,&mut self.diagonal_callbacks, position, side, out, allowedLength);
    }

    fn get_rook_possible_movements(&self, position: usize, side: Side, out: &mut Vec<usize>){
        Self::get_piece_orthogonal_movements(&self, position, side, out, 7);
        let (left_can_castle, right_can_castle) = self.can_castle(position);

        //Castle
        if(self.curr_turn == Side::White){
             if(position == 0 && left_can_castle){
                out.push(5);
             }
             else if(position == 7 && right_can_castle){
                out.push(5);
             }
        }
        else{
            if(position == 63 && right_can_castle){
                out.push(60);
             }
             else if(position == 56 && left_can_castle){
                out.push(60);
             }
        }
    }

    fn get_king_possible_movements(&self, position: usize, side: Side, out: &mut Vec<usize>){
        Self::get_piece_orthogonal_movements(&self, position, side, out, 1);
        Self::get_piece_diagonal_movements(&self, position, side, out, 1);
    }
    
    fn get_queen_possible_movements(&self, position: usize, side: Side, out: &mut Vec<usize>){
        Self::get_piece_orthogonal_movements(&self, position, side, out, 7);
        Self::get_piece_diagonal_movements(&self, position, side, out, 7);
    }

    fn get_knight_possible_movements(&self, position: usize, side: Side, out: &mut Vec<usize>){
        if(!is_pos_on_left_edge(position)){
            if position >= 17 && self.board_pieces_sides[position - 17] != side {
                out.push(position - 17);
            }
            if position <= 46 && self.board_pieces_sides[position + 17] != side {
                out.push(position + 17);
            }
        }

        if(!is_pos_on_right_edge(position)){
            if position >= 15 && self.board_pieces_sides[position - 15] != side {
                out.push(position - 15);
            }
            if position <= 48 && self.board_pieces_sides[position + 15] != side {
                out.push(position + 15);
            }
        }

        if position >= 10 && position % 8 >= 2 {
            if self.board_pieces_sides[position - 10] != side {
                out.push(position - 10);
            }
        }
        if position <= 53 && position % 8 <= 6 {
            if self.board_pieces_sides[position + 10] != side {
                out.push(position + 10);
            }
        }

        if position >= 6 && position % 8 <= 6 {
            if self.board_pieces_sides[position - 6] != side {
                out.push(position - 6);
            }
        }
        if position <= 57 && position % 8 >= 2 {
            if self.board_pieces_sides[position + 6] != side {
                out.push(position + 6);
            }
        }

    }

    fn get_bishop_possible_movements(&self, position: usize, side: Side, out: &mut Vec<usize>){
        Self::get_directions_movements(&self, &mut self.orthagonal_callbacks, position, side, out, 8);
        Self::get_piece_diagonal_movements(&self, position, side, out, 7);
    }

    fn get_pawn_possible_movements(&self, position: usize, side: Side, out: &mut Vec<usize>){
        let has_pawn_moved = Self::has_pawn_moved(position, side);

        if side == Side::White{
            if(position < 56){
                if self.board_pieces[position+8] == Piece::None{
                    out.push(position+8);
                    if !has_pawn_moved && self.board_pieces[position+16] == Piece::None{
                        out.push(position+16);
                    }
                }
                if !is_pos_on_left_edge(position) && self.board_pieces_sides[position+7] == Side::Black{
                    out.push(position+7);
                }
                if !is_pos_on_left_edge(position) && self.board_pieces_sides[position+9] == Side::Black{
                    out.push(position+9);
                }
            }
        }
        else if position > 7{
            if self.board_pieces[position-8] == Piece::None{
                out.push(position-8);
                if !has_pawn_moved && self.board_pieces[position-16] == Piece::None{
                    out.push(position-16);
                }
            }
            if !is_pos_on_left_edge(position) && self.board_pieces_sides[position-7] == Side::White{
                out.push(position-7);
            }
            if !is_pos_on_left_edge(position) && self.board_pieces_sides[position-9] == Side::White{
                out.push(position-9);
            }
        }
    }

    pub fn get_position_possible_movements(&self, position: usize) -> Vec<usize>{
        let pos_side: Side = self.board_pieces_sides[position];
        let pos_piece: Piece = self.board_pieces[position];
        let mut to_return = Vec::new();

        match pos_piece{
            Piece::Pawn => Self::get_pawn_possible_movements(self, position, pos_side, &mut to_return),
            Piece::Rook => Self::get_rook_possible_movements(self, position, pos_side, &mut to_return),
            Piece::Knight => Self::get_knight_possible_movements(self, position, pos_side, &mut to_return),
            Piece::Queen => Self::get_queen_possible_movements(self, position, pos_side, &mut to_return),
            Piece::King => Self::get_king_possible_movements(self, position, pos_side, &mut to_return),
            Piece::Bishop => Self::get_bishop_possible_movements(self, position, pos_side, &mut to_return),
            _ => return to_return
        }
        return to_return;
    }

    fn can_castle(&self, position: usize) -> (bool, bool){
        let castle_info = &self.castle_info;
        let mut can_castle_left_rook = false;
        let mut can_castle_right_rook = false;
        if(self.curr_turn == Side::White && !castle_info.top_king_moved){
            can_castle_left_rook = !castle_info.top_left_rook_moved;
            can_castle_right_rook = !castle_info.top_right_rook_moved;
        }
        else if(!castle_info.bottom_king_moved){
            can_castle_left_rook = !castle_info.bottom_left_rook_moved;
            can_castle_right_rook = !castle_info.bottom_right_rook_moved;
        }
        return (can_castle_left_rook, can_castle_right_rook);
    }

    fn is_checked(&self) -> bool{
        return false;
    }
    
     fn is_checked_mate(&self) -> bool{
        return false;
    }

    pub fn get_board_state(&mut self) -> BoardState{
        if self.is_checked_mate(){
            if(self.curr_turn == Side::White){
                return BoardState::WhiteLoseByCheckMate;
            }
            else{
                return BoardState::BlackLoseByCheckMate;
            }
        }
        else if self.is_checked(){
            if(self.curr_turn == Side::White){
                return BoardState::WhiteChecked;
            }
            else{
                return BoardState::WhiteChecked;
            }
        }
        return BoardState::Default;
    }

    pub fn handle_castling_logic(rook_one_start: usize, rook_two_start: usize, king_start: usize, origin: usize, target: usize, rook_one_moved: &mut bool, rook_two_moved: &mut bool, king_moved: &mut bool){
        let mut origin_is_rook_in_start_pos = false;
        match origin {
            rook_one_start => { *rook_one_moved = true; origin_is_rook_in_start_pos = true; }
            rook_two_start => { *rook_two_moved = true; origin_is_rook_in_start_pos = true; }
            king_start => *king_moved = true,
        }       
        if(origin_is_rook_in_start_pos && target == king_start){

        }
    }
    
    pub fn do_move(&mut self, origin: usize, target: usize) -> BoardState {
        let castle_info = &mut self.castle_info;

        if(self.curr_turn == Side::White){
            Self::handle_castling_logic(0, 7, 4, origin, target, &mut castle_info.top_left_rook_moved, &mut castle_info.top_right_rook_moved, &mut castle_info.top_king_moved);
            self.curr_turn = Side::Black;
        }
        else{
            Self::handle_castling_logic(56, 63, 60, origin, target, &mut castle_info.bottom_left_rook_moved, &mut castle_info.bottom_right_rook_moved, &mut castle_info.bottom_king_moved);
            self.curr_turn = Side::White;
        }
        if(self.board_pieces[target] != Piece::None || self.board_pieces[origin] == Piece::Pawn){
            self.fifty_move_rule = 50;
        }
        else if self.fifty_move_rule == 0 {
            return BoardState::Draw;
        }
        else{
            self.fifty_move_rule -= 1;
        }

        self.board_pieces[target] = self.board_pieces[origin];
        self.board_pieces_sides[target] = self.board_pieces_sides[origin];
        self.board_pieces[origin] = Piece::None;
        self.board_pieces_sides[origin] = Side::None;

        return self.get_board_state();
    }
}
