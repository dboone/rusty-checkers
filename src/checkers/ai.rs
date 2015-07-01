use checkers::player::Player;

pub enum Direction {
	/// The piece is moving such that its rank is increasing
	IncreasingRank,

	/// The piece is moving such that its rank is decreasing
	DecreasingRank
}

// A move from one tile to an adjacent diagonal one
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct SimpleMove {
	to_row : usize,
	to_col : usize
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct JumpMove {
	from_row : usize,
	from_col : usize,
	jumps : Vec<JumpMove>
}

/// Given the position of a main piece on a board, and the
/// direction this man piece is moving, determines the simple
/// moves available to this piece.
pub fn find_simple_moves_for_man
(board : &super::Board,
		direction : Direction,
		row : usize,
		col : usize)
-> Vec<SimpleMove> {
	let row_offset = match direction {
		Direction::IncreasingRank => TileOffset::Negative(1),
		Direction::DecreasingRank => TileOffset::Positive(1),
	};
	
	let mut moves = Vec::new();

	{
		let col_offset = TileOffset::Negative(1);
		push_simple_move_if_valid(
			board, row, col, &row_offset, &col_offset, &mut moves);
	}

	{
		let col_offset = TileOffset::Positive(1);
		push_simple_move_if_valid(
			board, row, col, &row_offset, &col_offset, &mut moves);
	}

	moves
}

/// Given the position of a main piece on a board, and the
/// direction this man piece is moving, determines the simple
/// moves available to this piece.
pub fn find_jump_moves_for_man
(board : &super::Board,
		player : &Player,
		direction : &Direction,
		row : usize,
		col : usize)
-> JumpMove {
	let mut jump_root = JumpMove {
		from_row : row,
		from_col : col,
		jumps : Vec::new()
	};

	let (pwnd_row_offset, jump_row_offset) = get_row_offsets(direction);

	find_jump_moves_for_man_rustcursive(
		board, player, &pwnd_row_offset, &jump_row_offset, &mut jump_root);

	jump_root
}

fn find_jump_moves_for_man_rustcursive
(board : &super::Board,
		player : &Player,
		pwnd_row_offset : &TileOffset,
		jump_row_offset : &TileOffset,
		jumps : &mut JumpMove) {	
	let col_offset_left = (TileOffset::Negative(1), TileOffset::Negative(2));
	let col_offset_right = (TileOffset::Positive(1), TileOffset::Positive(2));

	try_jump_moves_for_man(
		board, player, &pwnd_row_offset, &jump_row_offset, col_offset_left, jumps);
	try_jump_moves_for_man(
		board, player, &pwnd_row_offset, &jump_row_offset, col_offset_right, jumps);
}

fn try_jump_moves_for_man
(board : &super::Board,
		player : &Player,
		pwnd_row_offset : &TileOffset,
		jump_row_offset : &TileOffset,
		col_offset : (TileOffset, TileOffset),
		jumps : &mut JumpMove) {
	let start_row = jumps.from_row;
	let start_col = jumps.from_col;
	let (pwnd_col_offset, jump_col_offset) = col_offset;
	
	let tile_on_board = is_tile_offset_in_bounds(
		board, start_row, start_col, &jump_row_offset, &jump_col_offset);

	if !tile_on_board {
		return;
	}

	let (offset_row, offset_col)
		= offset_tile(start_row, start_col, &pwnd_row_offset, &pwnd_col_offset);
	let pwnd_tile = board.get_tile(offset_row, offset_col);

	let (offset_row, offset_col)
		= offset_tile(start_row, start_col, &jump_row_offset, &jump_col_offset);
	let jump_tile = board.get_tile(offset_row, offset_col);

	if jump_tile.get_piece().is_some() {
		return;
	}

	let pwnd_piece_enemy = pwnd_tile
		.get_piece()
		.map(|piece| piece.get_player_id() != player.id)
		.unwrap_or(false);

	if !pwnd_piece_enemy {
		return;
	}

	let mut the_move = JumpMove {
		from_row : offset_row,
		from_col : offset_col,
		jumps : Vec::new() };

	find_jump_moves_for_man_rustcursive(
		board, player, &pwnd_row_offset, &jump_row_offset, &mut the_move);

	jumps.jumps.push(the_move);
}

fn get_row_offsets(direction : &Direction) -> (TileOffset, TileOffset) {
	let (pwnd_row_offset, jump_row_offset) = match *direction {
			Direction::IncreasingRank =>
				(TileOffset::Negative(1), TileOffset::Negative(2)),
			Direction::DecreasingRank =>
				(TileOffset::Positive(1), TileOffset::Positive(2))
	};

	(pwnd_row_offset, jump_row_offset) 
}

/// Given the position of a king piece on a board, determines
/// the simple moves available to this piece.
///
/// This function does not require a Direction like the
/// find_simple_moves_for_man function, because kings can move
/// in all directions.
pub fn find_simple_moves_for_king
(board : &super::Board,
		row : usize,
		col : usize)
-> Vec<SimpleMove> {
	let mut moves = Vec::new();
	
	{
		let row_offset = TileOffset::Negative(1);
		let col_offset = TileOffset::Negative(1);
		push_simple_move_if_valid(
			board, row, col, &row_offset, &col_offset, &mut moves);
	}
	
	{
		let row_offset = TileOffset::Negative(1);
		let col_offset = TileOffset::Positive(1);
		push_simple_move_if_valid(
			board, row, col, &row_offset, &col_offset, &mut moves);
	}
	
	{
		let row_offset = TileOffset::Positive(1);
		let col_offset = TileOffset::Negative(1);
		push_simple_move_if_valid(
			board, row, col, &row_offset, &col_offset, &mut moves);
	}
	
	{
		let row_offset = TileOffset::Positive(1);
		let col_offset = TileOffset::Positive(1);
		push_simple_move_if_valid(
			board, row, col, &row_offset, &col_offset, &mut moves);
	}

	moves
}

// checks if it is possible to make a simple move with the given row
// and tile offset, and if so, adds the move to the vector
fn push_simple_move_if_valid
(board : &super::Board,
		start_row : usize,
		start_col : usize,
		row_offset : &TileOffset,
		col_offset : &TileOffset,
		moves : &mut Vec<SimpleMove>) {
	let tile_on_board = is_tile_offset_in_bounds(
		board, start_row, start_col, &row_offset, &col_offset);
	if tile_on_board {
		let (offset_row, offset_col)
			= offset_tile(start_row, start_col, &row_offset, &col_offset);
		let tile = board.get_tile(offset_row, offset_col);
		if tile.get_piece().is_none() {
			let the_move = SimpleMove{
				to_row : offset_row, to_col : offset_col };
			moves.push(the_move);
		}
	}
}

// This enum describes an offset direction and magnitude.
enum TileOffset {
	Positive(usize),
	Negative(usize)
}

// offsets a value based on the given offset direction and magnitude 
fn offset_value
(start_value : usize, value_offset : &TileOffset)
-> usize {
	match *value_offset {
	TileOffset::Negative(magnitude) => start_value - magnitude,
	TileOffset::Positive(magnitude) => start_value + magnitude,
	}
}

// Offsets a tile based on the given offset direction
// and magnitude in the row and column dimensions.
//
// Returns a 2 element tuple, where the first element
// is the offset row, and the second element is the
// offset column.
fn offset_tile
(start_row : usize,
		start_col : usize,
		row_offset : &TileOffset,
		col_offset : &TileOffset)
-> (usize, usize) {
	(offset_value(start_row, row_offset),
			offset_value(start_col, col_offset))
}

// checks if a value is in the given range using the given offset
//TODO maybe a range object can be used here as a param instead
// of the start and max values
fn is_offset_value_in_range
(start_value : usize,
		max_value : usize,
		value_offset : &TileOffset)
-> bool {
	match *value_offset {
	TileOffset::Negative(magnitude) => start_value >= magnitude,
	TileOffset::Positive(magnitude) => start_value + magnitude <= max_value
	}
}

// checks if a tile on the board can be reached when
// moving from one position on the board to another
fn is_tile_offset_in_bounds
(board : &super::Board,
		start_row : usize,
		start_col : usize,
		row_offset : &TileOffset,
		col_offset : &TileOffset)
-> bool {
	let max_row_index = board.number_rows() - 1;
	let max_col_index = board.number_columns() - 1;
	
	is_offset_value_in_range(start_row, max_row_index, row_offset)
	&& is_offset_value_in_range(start_col, max_col_index, col_offset)
}



//TODO could use some parameterized tests here
//TODO may be able to remove these tests onece the public API for this module is in place

#[test]
#[cfg(test)]
fn offset_value_negative_offset_1() {
	let offset = TileOffset::Negative(2);
	let result = offset_value(5, &offset);
	assert_eq!(3, result);
}

#[test]
#[cfg(test)]
fn offset_value_negative_offset_2() {
	let offset = TileOffset::Negative(1);
	let result = offset_value(3, &offset);
	assert_eq!(2, result);
}

#[test]
#[cfg(test)]
fn offset_value_positive_offset_1() {
	let offset = TileOffset::Positive(1);
	let result = offset_value(0, &offset);
	assert_eq!(1, result);
}

#[test]
#[cfg(test)]
fn offset_value_positive_offset_2() {
	let offset = TileOffset::Positive(2);
	let result = offset_value(5, &offset);
	assert_eq!(7, result);
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_positive_zero_offset() {
	let offset = TileOffset::Positive(0);
	assert!(is_offset_value_in_range(0, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_negative_zero_offset() {
	let offset = TileOffset::Negative(0);
	assert!(is_offset_value_in_range(0, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_valid_positive_offset() {
	let offset = TileOffset::Positive(2);
	assert!(is_offset_value_in_range(5, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_invalid_positive_offset() {
	let offset = TileOffset::Positive(2);
	assert!(!is_offset_value_in_range(6, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_valid_negative_offset() {
	let offset = TileOffset::Negative(2);
	assert!(is_offset_value_in_range(2, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_invalid_negative_offset() {
	let offset = TileOffset::Negative(2);
	assert!(!is_offset_value_in_range(1, 7, &offset));
}

#[cfg(test)]
mod test {

mod simple_move {

mod man_piece {

use super::super::super::*;
use checkers::Board;
use checkers::ManPiece;
use checkers::OccupiedTile;
use checkers::Player;

#[test]
fn no_moves_with_single_tile_board() {
	let board = Board::new(1, 1);
	let result = find_simple_moves_for_man(
		&board, Direction::IncreasingRank, 0, 0);
	assert_eq!(Vec::<SimpleMove>::new(), result);
}

#[test]
fn no_moves_when_min_rank_and_decreasing_rank() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_man(
		&board, Direction::DecreasingRank, 7, 4);
	assert_eq!(Vec::<SimpleMove>::new(), result);
}

#[test]
fn no_moves_when_max_rank_and_increasing_rank() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_man(
		&board, Direction::IncreasingRank, 0, 4);
	assert_eq!(Vec::<SimpleMove>::new(), result);
}

#[test]
fn single_move_when_min_file() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_man(
		&board, Direction::IncreasingRank, 4, 0);
	assert_eq!(
		vec![SimpleMove{to_row : 3, to_col : 1}],
		result);
}

#[test]
fn single_move_when_max_file() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_man(
		&board, Direction::DecreasingRank, 3, 7);
	assert_eq!(
		vec![SimpleMove{to_row : 4, to_col : 6}],
		result);
}

#[test]
fn two_moves_when_middle_of_board_1() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_man(
		&board, Direction::DecreasingRank, 3, 5);
	assert_eq!(
		vec![
			SimpleMove{to_row : 4, to_col : 4},
			SimpleMove{to_row : 4, to_col : 6}],
		result);
}

#[test]
fn two_moves_when_middle_of_board_2() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_man(
		&board, Direction::IncreasingRank, 1, 2);
	assert_eq!(
		vec![
			SimpleMove{to_row : 0, to_col : 1},
			SimpleMove{to_row : 0, to_col : 3}],
		result);
}

#[test]
fn move_blocked_when_tile_occupied_1() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0};
	let piece = ManPiece::new(&player);
	let tile = OccupiedTile::new(Box::new(piece));
	board.set_tile(3, 3, Box::new(tile));
	
	let result = find_simple_moves_for_man(
		&board, Direction::IncreasingRank, 4, 4);
	assert_eq!(
		vec![SimpleMove{to_row : 3, to_col : 5}],
		result);
}

#[test]
fn move_blocked_when_tile_occupied_2() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0};
	let piece = ManPiece::new(&player);
	let tile = OccupiedTile::new(Box::new(piece));
	board.set_tile(3, 5, Box::new(tile));
	
	let result = find_simple_moves_for_man(
		&board, Direction::IncreasingRank, 4, 4);
	assert_eq!(
		vec![SimpleMove{to_row : 3, to_col : 3}],
		result);
}

}

mod king_piece {

use super::super::super::*;
use checkers::Board;
use checkers::ManPiece;
use checkers::OccupiedTile;
use checkers::Player;

#[test]
fn single_tile_board_has_no_moves() {
	let board = Board::new(1, 1);
	let result = find_simple_moves_for_king(
		&board, 0, 0);
	assert_eq!(Vec::<SimpleMove>::new(), result);
}

#[test]
fn single_move_when_min_rank_and_min_file() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_king(
		&board, 7, 0);
	assert_eq!(
		vec![SimpleMove{to_row : 6, to_col : 1}],
		result);
}

#[test]
fn single_move_when_min_rank_and_max_file() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_king(
		&board, 7, 7);
	assert_eq!(
		vec![SimpleMove{to_row : 6, to_col : 6}],
		result);
}

#[test]
fn single_move_when_max_rank_and_min_file() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_king(
		&board, 0, 0);
	assert_eq!(
		vec![SimpleMove{to_row : 1, to_col : 1}],
		result);
}

#[test]
fn single_move_when_max_rank_and_max_file() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_king(
		&board, 0, 7);
	assert_eq!(
		vec![SimpleMove{to_row : 1, to_col : 6}],
		result);
}

#[test]
fn four_moves_when_middle_of_board() {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_king(
		&board, 3, 5);
	assert_eq!(
		vec![
			SimpleMove{to_row : 2, to_col : 4},
			SimpleMove{to_row : 2, to_col : 6},
			SimpleMove{to_row : 4, to_col : 4},
			SimpleMove{to_row : 4, to_col : 6}],
		result);
}

#[test]
fn move_blocked_when_tile_occupied_1() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0};
	let piece = ManPiece::new(&player);
	let tile = OccupiedTile::new(Box::new(piece));
	board.set_tile(2, 4, Box::new(tile));
	
	let result = find_simple_moves_for_king(
		&board, 3, 5);
	assert_eq!(
		vec![
			SimpleMove{to_row : 2, to_col : 6},
			SimpleMove{to_row : 4, to_col : 4},
			SimpleMove{to_row : 4, to_col : 6}],
		result);
}

#[test]
fn move_blocked_when_tile_occupied_2() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0};
	let piece = ManPiece::new(&player);
	let tile = OccupiedTile::new(Box::new(piece));
	board.set_tile(2, 6, Box::new(tile));
	
	let result = find_simple_moves_for_king(
		&board, 3, 5);
	assert_eq!(
		vec![
			SimpleMove{to_row : 2, to_col : 4},
			SimpleMove{to_row : 4, to_col : 4},
			SimpleMove{to_row : 4, to_col : 6}],
		result);
}

#[test]
fn move_blocked_when_tile_occupied_3() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0};
	let piece = ManPiece::new(&player);
	let tile = OccupiedTile::new(Box::new(piece));
	board.set_tile(4, 4, Box::new(tile));
	
	let result = find_simple_moves_for_king(
		&board, 3, 5);
	assert_eq!(
		vec![
			SimpleMove{to_row : 2, to_col : 4},
			SimpleMove{to_row : 2, to_col : 6},
			SimpleMove{to_row : 4, to_col : 6}],
		result);
}

#[test]
fn move_blocked_when_tile_occupied_4() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0};
	let piece = ManPiece::new(&player);
	let tile = OccupiedTile::new(Box::new(piece));
	board.set_tile(4, 6, Box::new(tile));
	
	let result = find_simple_moves_for_king(
		&board, 3, 5);
	assert_eq!(
		vec![
			SimpleMove{to_row : 2, to_col : 4},
			SimpleMove{to_row : 2, to_col : 6},
			SimpleMove{to_row : 4, to_col : 4}],
		result);
}

}

}

}