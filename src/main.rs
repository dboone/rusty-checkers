#[macro_use]
extern crate corrosion;
use std::io::stdout;
use std::io::Write;

mod checkers;
use checkers::Board;
use checkers::Player;

const EMPTY_PIECE_STR : &'static str = " ";
const OCCUPIED_PIECE_STR : &'static str = "O";

fn print_board<TWrite : Write>(writer : &mut TWrite, board : &Board) -> Result<(), std::io::Error> {
	for c in 0..board.number_columns() {
		for r in 0..board.number_rows() {
			let tile = board.get_tile(r, c);
			let piece_str = match tile.get_piece() {
				None => EMPTY_PIECE_STR,
				Some(_) => OCCUPIED_PIECE_STR
			};
			
			try!(write!(writer, "[{}]", piece_str));
		}
		try!(writeln!(writer, ""));
	}
	
	Ok(())
}

fn main() {
    println!("Welcome to Draughts!");

	let player1 = Player{ id : 0 };
	let player2 = Player{ id : 1 };
    let board = Board::new_checkerboard(&player1, &player2);
	let mut writer = stdout();
	print_board(&mut writer, &board).unwrap();
}
