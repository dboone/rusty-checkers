extern crate std;

use std::char;
use std::io;
use std::io::Write;
use checkers::Board;
use checkers::piece::PieceType;

const EMPTY_PIECE_STR : &'static str = " ";

const RED_MAN_STR : &'static str = "r";
const RED_KING_STR : &'static str = "R";
const BLACK_MAN_STR : &'static str = "b";
const BLACK_KING_STR : &'static str = "B";

fn print_justified_file
<TWrite : Write>
(writer : &mut TWrite, columns : usize, padding : usize)
-> Result<(), std::io::Error> {
    for _ in 0..padding {
        try!(write!(writer, " "));
    }

	let initial_file = 'A' as u32;
    for c in 0..columns {
        let file = char::from_u32(initial_file + c as u32).unwrap();
        try!(write!(writer, "  {}", file));
    }

    try!(writeln!(writer, ""));

    Ok(())
}

fn print_justified_rank
<TWrite : Write>
(writer : &mut TWrite, rank : usize, padding : usize)
-> Result<(), io::Error> {
    let cur_rank = rank.to_string();

    for _ in 0..padding - cur_rank.len() {
        try!(write!(writer, " "));
    }
    try!(write!(writer, "{} ", cur_rank));

    Ok(())
}

pub fn print_board
<TWrite : Write>
(writer : &mut TWrite, board : &Board)
-> Result<(), io::Error> {
    let file_padding = board.number_columns().to_string().len();
    let rank_padding = board.number_rows().to_string().len();

    try!(print_justified_file(writer, board.number_columns(), file_padding));

	for r in (0..board.number_rows()).rev() {
        try!(print_justified_rank(writer, r + 1, rank_padding));
		for c in 0..board.number_columns() {
			let tile = board.get_tile(r, c);
			let piece_str = match tile.get_piece() {
				None => EMPTY_PIECE_STR,
				Some(piece) =>
					match (piece.get_type(), piece.get_player_id()) {
						(PieceType::Man, 1) => RED_MAN_STR,
						(PieceType::King, 1) => RED_KING_STR,
						(PieceType::Man, 2) => BLACK_MAN_STR,
						(PieceType::King, 2) => BLACK_KING_STR,
						_ => unreachable!()
					}
			};
			
			try!(write!(writer, "[{}]", piece_str));
		}
		try!(writeln!(writer, " {}", r + 1));
	}

    try!(print_justified_file(writer, board.number_columns(), file_padding));
	Ok(())
}

#[cfg(test)]
mod test {
	use super::*;
	
	use checkers::Board;
	use checkers::piece::{KingPiece, ManPiece};
	use checkers::Player;
	use checkers::tile::OccupiedTile;
	
	#[test]
	fn empty_1x1_board() {
		let board = Board::new(1, 1);
		
		let mut result = Vec::<u8>::new();
		print_board(&mut result, &board);
		
		let exp_result = "   A\n1 [ ] 1\n   A\n";
		
		assert_eq!(exp_result.as_bytes(), &*result);
	}
	
	#[test]
	fn empty_3x3_board() {
		let board = Board::new(3, 3);
		
		let mut result = Vec::<u8>::new();
		print_board(&mut result, &board);
		
		let exp_result = concat!(
			"   A  B  C\n",
			"3 [ ][ ][ ] 3\n",
			"2 [ ][ ][ ] 2\n",
			"1 [ ][ ][ ] 1\n",
			"   A  B  C\n");
   
		assert_eq!(exp_result.as_bytes(), &*result);
	}
	
	#[test]
	fn empty_5x3_board() {
		let board = Board::new(5, 3);
		
		let mut result = Vec::<u8>::new();
		print_board(&mut result, &board);
		
		let exp_result = concat!(
			"   A  B  C\n",
			"5 [ ][ ][ ] 5\n",
			"4 [ ][ ][ ] 4\n",
			"3 [ ][ ][ ] 3\n",
			"2 [ ][ ][ ] 2\n",
			"1 [ ][ ][ ] 1\n",
			"   A  B  C\n");
   
		assert_eq!(exp_result.as_bytes(), &*result);
	}
	
	#[test]
	fn board_with_pieces() {
		let mut result = Vec::<u8>::new();
			
		let red_player = Player{id : 1};
		let black_player = Player{id : 2};
		
		let mut board = Board::new(5, 3);
		
		let red_man = ManPiece::new(&red_player);
		let red_king = KingPiece::new(&red_player);
		let black_man = ManPiece::new(&black_player);
		let black_king = KingPiece::new(&black_player);
		board.set_tile(0, 0, Box::new(OccupiedTile::new(Box::new(red_man))));
		board.set_tile(4, 2, Box::new(OccupiedTile::new(Box::new(red_king))));
		board.set_tile(0, 2, Box::new(OccupiedTile::new(Box::new(black_man))));
		board.set_tile(4, 0, Box::new(OccupiedTile::new(Box::new(black_king))));
		
		print_board(&mut result, &board);

		let exp_result = concat!(
			"   A  B  C\n",
			"5 [B][ ][R] 5\n",
			"4 [ ][ ][ ] 4\n",
			"3 [ ][ ][ ] 3\n",
			"2 [ ][ ][ ] 2\n",
			"1 [r][ ][b] 1\n",
			"   A  B  C\n");
		
		assert_eq!(exp_result.as_bytes(), &*result);
	}
}