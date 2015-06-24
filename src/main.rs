#[macro_use]
extern crate corrosion;
use std::io::stdout;
use std::io::Write;
use std::char;

mod checkers;
use checkers::Board;
use checkers::Player;

const EMPTY_PIECE_STR : &'static str = " ";
const OCCUPIED_PIECE_STR : &'static str = "O";

fn print_justified_file<TWrite : Write>(writer : &mut TWrite, columns : usize, padding : usize) -> Result<(), std::io::Error> {
    for _ in 0..padding + 1 {
        try!(write!(writer, " "));
    }

    for c in 0..columns {
        let file = char::from_u32('A' as u32 + c as u32).unwrap();
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

    print_justified_file(writer, board.number_columns(), file_padding).unwrap();

	for c in 0..board.number_columns() {
        print_justified_rank(writer, c + 1, rank_padding).unwrap();
		for r in 0..board.number_rows() {
			let tile = board.get_tile(r, c);
			let piece_str = match tile.get_piece() {
				None => EMPTY_PIECE_STR,
				Some(_) => OCCUPIED_PIECE_STR
			};
			
			try!(write!(writer, "[{}]", piece_str));
		}
		try!(writeln!(writer, " {} ", c + 1));
	}

    print_justified_file(writer, board.number_columns(), file_padding).unwrap();	
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
