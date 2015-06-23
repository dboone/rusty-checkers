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

fn print_file<TWrite : Write>(writer : &mut TWrite, columns : usize) -> Result<(), std::io::Error> {
    try!(write!(writer, "  "));

    for c in 0..columns {
        try!(write!(writer, " {} ", char::from_u32(65 + c as u32).unwrap()));
    }

    try!(writeln!(writer, ""));

    Ok(())
}

fn print_justified_rank<TWrite : Write>(writer : &mut TWrite, rank : usize, rows : usize) -> Result<(), std::io::Error> {
    let cur_width = rank.to_string().len();
    let max_width = rows.to_string().len();

    for _ in 0..max_width - cur_width {
        try!(write!(writer, " "));
    }
    try!(write!(writer, "{} ", rank));

    Ok(())
}

fn print_board<TWrite : Write>(writer : &mut TWrite, board : &Board) -> Result<(), std::io::Error> {
    print_file(writer, board.number_columns()).unwrap();
	for c in 0..board.number_columns() {
        print_justified_rank(writer, c + 1, board.number_rows()).unwrap();
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

    print_file(writer, board.number_columns()).unwrap();	
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
