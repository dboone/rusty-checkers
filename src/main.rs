#[macro_use]
extern crate corrosion;
use std::io::stdout;
use std::io::Write;
use std::char;

mod checkers;
use checkers::Board;
use checkers::Game;
use checkers::GameState;
use checkers::MoveError;
use checkers::SimpleMove;

const EMPTY_PIECE_STR : &'static str = " ";
const OCCUPIED_PIECE_STR : &'static str = "O";

fn print_justified_file<TWrite : Write>(writer : &mut TWrite, columns : usize, padding : usize) -> Result<(), std::io::Error> {
    for _ in 0..padding + 1 {
        try!(write!(writer, " "));
    }

	let initial_file = 'A' as u32;
    for c in 0..columns {
        let file = char::from_u32(initial_file + c as u32).unwrap();
        try!(write!(writer, " {} ", file));
    }

    try!(writeln!(writer, ""));

    Ok(())
}

fn print_justified_rank<TWrite : Write>(writer : &mut TWrite, rank : usize, padding : usize) -> Result<(), std::io::Error> {
    let cur_rank = rank.to_string();

    for _ in 0..padding - cur_rank.len() {
        try!(write!(writer, " "));
    }
    try!(write!(writer, "{} ", cur_rank));

    Ok(())
}

fn print_board<TWrite : Write>(writer : &mut TWrite, board : &Board) -> Result<(), std::io::Error> {
    let file_padding = board.number_columns().to_string().len();
    let rank_padding = board.number_rows().to_string().len();

    try!(print_justified_file(writer, board.number_columns(), file_padding));

	for r in (0..board.number_rows()).rev() {
        try!(print_justified_rank(writer, r + 1, rank_padding));
		for c in 0..board.number_columns() {
			let tile = board.get_tile(r, c);
			let piece_str = match tile.get_piece() {
				None => EMPTY_PIECE_STR,
				Some(_) => OCCUPIED_PIECE_STR
			};
			
			try!(write!(writer, "[{}]", piece_str));
		}
		try!(writeln!(writer, " {} ", r + 1));
	}

    try!(print_justified_file(writer, board.number_columns(), file_padding));
	Ok(())
}

fn main() {
    println!("Welcome to Draughts!");

	let game = Game::new();
	match game.apply_simple_move(SimpleMove::new(3, 1)) {
		Ok(state) => 
			match state {
				GameState::InProgress => { },
				GameState::GameOver => { }
			},
		Err(error) =>
			match error {
				MoveError::InvalidMove => { },
				MoveError::ShouldHaveJumped => { }
			}
	}

	let mut writer = stdout();
	print_board(&mut writer, game.board()).unwrap();
}
