use caspervk_chess::Game;

mod debugutils;

pub use debugutils::visualise_board;



fn print_pos_possible_movements(game: &Game, pos: usize){
    let possible_movements: Vec<usize> = game.get_position_possible_movements(pos);
    for i in 0..possible_movements.len(){
        println!("{:?} ", possible_movements[i]);
    }
}
fn main(){
    let mut game = Game::new();

    game.do_move(52, 36);
    visualise_board(game.board_pieces, game.board_pieces_sides);
    print_pos_possible_movements(&game, 62);

    game.do_move(62, 45);
    visualise_board(game.board_pieces, game.board_pieces_sides);
    print_pos_possible_movements(&game, 45);

    game.do_move(45, 39);
    visualise_board(game.board_pieces, game.board_pieces_sides);
    print_pos_possible_movements(&game, 39);
}