#[macro_use]
extern crate corrosion;
use std::io::stdout;

mod checkers;
use checkers::{
	BoardPosition,
	Game,
	GameState,
	MoveError,
	SimpleMove};

mod util;

fn apply_positions_as_move
(game : &mut Game, positions : Vec<BoardPosition>)
-> Result<GameState, MoveError> {
	if positions.len() == 2 {
		// simple move or jump move?
		let start = positions[0];
		let end = positions[1];
		
		let row_diff = util::absolute_diff(start.row, end.row);
		let col_diff = util::absolute_diff(start.column, end.column);
		
		if row_diff == 1 && col_diff == 1 {
			game.apply_simple_move(SimpleMove::new(
				start.row, start.column, end.row, end.column))
		} else {
			game.apply_jump_move(positions)
		}
	} else {
		game.apply_jump_move(positions)
	}
}

fn main() {
    println!("Welcome to Draughts!");

	let mut writer = stdout();
	let mut game = Game::new();

	checkers::display::print_board(&mut writer, game.board()).unwrap();

	let positions = vec![BoardPosition::new(2, 0), BoardPosition::new(3, 1)];
	let move_result = apply_positions_as_move(&mut game, positions);

	checkers::display::print_board(&mut writer, game.board()).unwrap();
}
