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
fn single_tile_board_has_no_moves() {
	let board = Board::new(1, 1);
	let result = find_simple_moves_for_man(
		&board, Direction::IncreasingRank, 0, 0);
	assert_eq!(Vec::<SimpleMove>::new(), result);
}

fn test_move
(dir : Direction,
		start_row : usize,
		start_col : usize,
		exp_result : Vec<SimpleMove>) {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_man(
		&board, dir, start_row, start_col);
	assert_eq!(exp_result, result);
}

ptest!(test_move [
	no_moves_when_min_rank_and_decreasing_rank(
		Direction::DecreasingRank, 7, 4, Vec::new()),
	
	no_moves_when_max_rank_and_increasing_rank(
		Direction::IncreasingRank, 0, 4, Vec::new()),
	
	single_move_when_min_file(
		Direction::IncreasingRank, 4, 0, vec![SimpleMove{to_row : 3, to_col : 1}]),
	
	single_move_when_max_file(
		Direction::DecreasingRank, 3, 7, vec![SimpleMove{to_row : 4, to_col : 6}]),
	
	two_moves_when_middle_of_board_1(
		Direction::DecreasingRank,
		3,
		5,
		vec![SimpleMove{to_row : 4, to_col : 4}, SimpleMove{to_row : 4, to_col : 6}]),
		
	two_moves_when_middle_of_board_2(
		Direction::IncreasingRank,
		1,
		2,
		vec![SimpleMove{to_row : 0, to_col : 1}, SimpleMove{to_row : 0, to_col : 3}])
]);

fn test_move_blocked
(piece_row : usize,
		piece_col : usize,
		dir : Direction,
		start_row : usize,
		start_col : usize,
		exp_result : Vec<SimpleMove>) {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0};
	let piece = ManPiece::new(&player);
	let tile = OccupiedTile::new(Box::new(piece));
	board.set_tile(piece_row, piece_col, Box::new(tile));
	
	let result = find_simple_moves_for_man(
		&board, dir, start_row, start_col);
	assert_eq!(exp_result, result);
}

ptest!(test_move_blocked [
	move_blocked_when_tile_occupied_1(
		3, 3, Direction::IncreasingRank, 4, 4, vec![SimpleMove{to_row : 3, to_col : 5}]),
		
	move_blocked_when_tile_occupied_2(
		3, 5, Direction::IncreasingRank, 4, 4, vec![SimpleMove{to_row : 3, to_col : 3}])
]);

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

fn test_move
(start_row : usize,
		start_col : usize,
		exp_result : Vec<SimpleMove>) {
	let board = Board::new(8, 8);
	let result = find_simple_moves_for_king(
		&board, start_row, start_col);
	assert_eq!(exp_result, result);
}

ptest!(test_move [
	single_move_when_min_rank_and_min_file(
		7, 0, vec![SimpleMove{to_row : 6, to_col : 1}]),
		
	single_move_when_min_rank_and_max_file(
		7, 7, vec![SimpleMove{to_row : 6, to_col : 6}]),
		
	single_move_when_max_rank_and_min_file(
		0, 0, vec![SimpleMove{to_row : 1, to_col : 1}]),
		
	single_move_when_max_rank_and_max_file(
		0, 7, vec![SimpleMove{to_row : 1, to_col : 6}]),
		
	four_moves_when_middle_of_board(
		3, 5, vec![
			SimpleMove{to_row : 2, to_col : 4},
			SimpleMove{to_row : 2, to_col : 6},
			SimpleMove{to_row : 4, to_col : 4},
			SimpleMove{to_row : 4, to_col : 6}])
]);

fn test_move_blocked
(piece_row : usize,
		piece_col : usize,
		start_row : usize,
		start_col : usize,
		exp_result : Vec<SimpleMove>) {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let piece = ManPiece::new(&player);
	let tile = OccupiedTile::new(Box::new(piece));
	board.set_tile(piece_row, piece_col, Box::new(tile));
	
	let result = find_simple_moves_for_king(
		&board, start_row, start_col);
	assert_eq!(exp_result, result);
}

ptest!(test_move_blocked [
	move_blocked_when_tile_occupied_1(
		2, 4, 3, 5, vec![
			SimpleMove{to_row : 2, to_col : 6},
			SimpleMove{to_row : 4, to_col : 4},
			SimpleMove{to_row : 4, to_col : 6}]),
	
	move_blocked_when_tile_occupied_2(
		2, 6, 3, 5, vec![
			SimpleMove{to_row : 2, to_col : 4},
			SimpleMove{to_row : 4, to_col : 4},
			SimpleMove{to_row : 4, to_col : 6}]),
			
	move_blocked_when_tile_occupied_3(
		4, 4, 3, 5, vec![
			SimpleMove{to_row : 2, to_col : 4},
			SimpleMove{to_row : 2, to_col : 6},
			SimpleMove{to_row : 4, to_col : 6}]),
			
	move_blocked_when_tile_occupied_4(
		4, 6, 3, 5, vec![
			SimpleMove{to_row : 2, to_col : 4},
			SimpleMove{to_row : 2, to_col : 6},
			SimpleMove{to_row : 4, to_col : 4}])
]);

}

}

mod jump_move {

mod man_piece {

use super::super::super::*;
use checkers::Board;
use checkers::ManPiece;
use checkers::OccupiedTile;
use checkers::Player;

fn test_jumping_alone
(start_row : usize, start_col : usize) {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::DecreasingRank;

	let result = find_jump_moves_for_man(
		&board, &player, &direction, start_row, start_col);

	let exp_result = JumpMove{
		from_row : start_row,
		from_col : start_col,
		jumps : Vec::new() };

	assert_eq!(exp_result, result);
}

ptest!(test_jumping_alone [
	jumping_left_off_board(6, 1),
	jumping_right_off_board(6, 6),
	jumping_middle_of_board(4, 3)
]);


fn test_single_jump_single_enemy
(start_row : usize,
		start_col : usize,
		enemy_row : usize,
		enemy_col : usize,
		exp_result : JumpMove) {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::DecreasingRank;

	let opponent = Player{ id : 1 };
	
	let enemy_piece = ManPiece::new(&opponent);
	let enemy_tile = OccupiedTile::new(Box::new(enemy_piece));
	board.set_tile(enemy_row, enemy_col, Box::new(enemy_tile));

	let result = find_jump_moves_for_man(
		&board, &player, &direction, start_row, start_col);

	assert_eq!(exp_result, result);
}

ptest!(test_single_jump_single_enemy [
	jumping_adjacent_enemy_left(
	4, 3, 5, 2, JumpMove{
		from_row : 4,
		from_col : 3,
		jumps : vec![JumpMove{ from_row : 6, from_col : 1, jumps : Vec::new() }] }),
		
	jumping_adjacent_enemy_right(
		4, 3, 5, 4, JumpMove{
			from_row : 4,
			from_col : 3,
			jumps : vec![JumpMove{ from_row : 6, from_col : 5, jumps : Vec::new() }] })
]);

fn test_single_jump_two_enemies
(start_row : usize,
		start_col : usize,
		left_enemy_row : usize,
		left_enemy_col : usize,
		right_enemy_row : usize,
		right_enemy_col : usize,
		exp_result : JumpMove) {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::DecreasingRank;

	let opponent = Player{ id : 1 };

	let left_enemy_piece = ManPiece::new(&opponent);
	let left_enemy_tile = OccupiedTile::new(Box::new(left_enemy_piece));
	board.set_tile(left_enemy_row, left_enemy_col, Box::new(left_enemy_tile));

	let right_enemy_piece = ManPiece::new(&opponent);
	let right_enemy_tile = OccupiedTile::new(Box::new(right_enemy_piece));
	board.set_tile(right_enemy_row, right_enemy_col, Box::new(right_enemy_tile));

	let result = find_jump_moves_for_man(
		&board, &player, &direction, start_row, start_col);

	assert_eq!(exp_result, result);
}
	
ptest!(test_single_jump_two_enemies [
	jumping_two_forward_adjacent_enemies(
		4, 3, 5, 2, 5, 4, JumpMove{
			from_row : 4,
			from_col : 3,
			jumps : vec![
				JumpMove{ from_row : 6, from_col : 1, jumps : Vec::new() },
				JumpMove{ from_row : 6, from_col : 5, jumps : Vec::new() }] }),
				
	jumping_two_backward_adjacent_enemies(
		6, 3, 5, 2, 5, 4, JumpMove{
			from_row : 6,
			from_col : 3,
			jumps : Vec::new() })
]);

fn test_jumping_friendly_piece
(start_row : usize,
		start_col : usize,
		friendly_row : usize,
		friendly_col : usize) {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::DecreasingRank;

	let left_piece = ManPiece::new(&player);
	let left_tile = OccupiedTile::new(Box::new(left_piece));
	board.set_tile(friendly_row, friendly_col, Box::new(left_tile));

	let result = find_jump_moves_for_man(
		&board, &player, &direction, start_row, start_col);

	let exp_result = JumpMove{
		from_row : start_row,
		from_col : start_col,
		jumps : Vec::new() };

	assert_eq!(exp_result, result);
}

ptest!(test_jumping_friendly_piece [
	jumping_adjacent_friendly_piece_left(6, 3, 5, 2),
	jumping_adjacent_friendly_piece_right(6, 3, 5, 4)
]);

fn test_single_jump_blocked
(start_row : usize,
		start_col : usize,
		pwnd_row : usize,
		pwnd_col : usize,
		blocked_row : usize,
		blocked_col : usize) {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::DecreasingRank;

	let opponent = Player{ id : 1 };
	let pwnd_piece = ManPiece::new(&opponent);
	let pwnd_tile = OccupiedTile::new(Box::new(pwnd_piece));
	board.set_tile(pwnd_row, pwnd_col, Box::new(pwnd_tile));

	let block_piece = ManPiece::new(&player);
	let block_tile = OccupiedTile::new(Box::new(block_piece));
	board.set_tile(blocked_row, blocked_col, Box::new(block_tile));

	let result = find_jump_moves_for_man(
		&board, &player, &direction, start_row, start_col);

	let exp_result = JumpMove{
		from_row : start_row,
		from_col : start_col,
		jumps : Vec::new() };

	assert_eq!(exp_result, result);
}

ptest!(test_single_jump_blocked [
	jumping_adjacent_enemy_blocked_left(4, 3, 5, 2, 6, 1),
	jumping_adjacent_enemy_blocked_right(4, 3, 5, 4, 6, 5)
]);

#[test]
fn jumping_two_forward_adjacent_enemies_left_blocked() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::DecreasingRank;
	let start_row = 4;
	let start_col = 3;

	let opponent = Player{ id : 1 };

	let left_piece = ManPiece::new(&opponent);
	let left_tile = OccupiedTile::new(Box::new(left_piece));
	board.set_tile(5, 2, Box::new(left_tile));

	let block_piece = ManPiece::new(&player);
	let block_tile = OccupiedTile::new(Box::new(block_piece));
	board.set_tile(6, 1, Box::new(block_tile));

	let right_piece = ManPiece::new(&opponent);
	let right_tile = OccupiedTile::new(Box::new(right_piece));
	board.set_tile(5, 4, Box::new(right_tile));

	let result = find_jump_moves_for_man(
		&board, &player, &direction, start_row, start_col);

	let exp_result = JumpMove{
		from_row : start_row,
		from_col : start_col,
		jumps : vec![ JumpMove{ from_row : 6, from_col : 5, jumps : Vec::new() } ] };

	assert_eq!(exp_result, result);
}

#[test]
fn jumping_two_forward_adjacent_enemies_right_blocked() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::DecreasingRank;
	let start_row = 4;
	let start_col = 3;

	let opponent = Player{ id : 1 };

	let left_piece = ManPiece::new(&opponent);
	let left_tile = OccupiedTile::new(Box::new(left_piece));
	board.set_tile(5, 2, Box::new(left_tile));

	let block_piece = ManPiece::new(&player);
	let block_tile = OccupiedTile::new(Box::new(block_piece));
	board.set_tile(6, 5, Box::new(block_tile));

	let right_piece = ManPiece::new(&opponent);
	let right_tile = OccupiedTile::new(Box::new(right_piece));
	board.set_tile(5, 4, Box::new(right_tile));

	let result = find_jump_moves_for_man(
		&board, &player, &direction, start_row, start_col);

	let exp_result = JumpMove{
		from_row : start_row,
		from_col : start_col,
		jumps : vec![ JumpMove{ from_row : 6, from_col : 1, jumps : Vec::new() } ] };

	assert_eq!(exp_result, result);
}

#[test]
fn the_one_true_test() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::IncreasingRank;

	let opponent = Player{ id : 1 };

	let piece12 = ManPiece::new(&opponent);
	let tile12 = OccupiedTile::new(Box::new(piece12));
	board.set_tile(1, 2, Box::new(tile12));

	let piece14 = ManPiece::new(&opponent);
	let tile14 = OccupiedTile::new(Box::new(piece14));
	board.set_tile(1, 4, Box::new(tile14));

	let piece32 = ManPiece::new(&opponent);
	let tile32 = OccupiedTile::new(Box::new(piece32));
	board.set_tile(3, 2, Box::new(tile32));

	let piece34 = ManPiece::new(&opponent);
	let tile34 = OccupiedTile::new(Box::new(piece34));
	board.set_tile(3, 4, Box::new(tile34));

	let piece36 = ManPiece::new(&opponent);
	let tile36 = OccupiedTile::new(Box::new(piece36));
	board.set_tile(3, 6, Box::new(tile36));

	let piece52 = ManPiece::new(&opponent);
	let tile52 = OccupiedTile::new(Box::new(piece52));
	board.set_tile(5, 2, Box::new(tile52));

	let piece54 = ManPiece::new(&opponent);
	let tile54 = OccupiedTile::new(Box::new(piece54));
	board.set_tile(5, 4, Box::new(tile54));

	let piece63 = ManPiece::new(&player);
	let tile63 = OccupiedTile::new(Box::new(piece63));
	board.set_tile(6, 3, Box::new(tile63));

	let start_row = 6;
	let start_col = 3;

	let result = find_jump_moves_for_man(
		&board, &player, &direction, start_row, start_col);

	let exp_result = JumpMove{
		from_row : start_row,
		from_col : start_col,
		jumps : vec![ JumpMove{
						 from_row: 4, from_col: 1, jumps:
							vec![ 
								JumpMove{
									from_row: 2, from_col: 3, jumps:
									vec![ JumpMove{ from_row: 0, from_col: 1, jumps: Vec::new() },
										  JumpMove{ from_row: 0, from_col: 5, jumps: Vec::new() }
										]
						        }
						    ]
					},
					JumpMove{
						from_row: 4, from_col: 5, jumps: 
							vec![ 
								JumpMove{
									from_row: 2, from_col: 3, jumps:
									vec![ JumpMove{ from_row: 0, from_col: 1, jumps: Vec::new() },
										  JumpMove{ from_row: 0, from_col: 5, jumps: Vec::new() } 
										]
								},
								JumpMove{ from_row: 2, from_col: 7, jumps: Vec::new() }
							]
					}
			]
	};

	assert_eq!(exp_result, result);
}

}

}

}
