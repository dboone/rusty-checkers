use checkers::player::Player;
use checkers::board::Board;

use std::collections::HashSet;

#[derive(Copy, Clone)]
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
	from_row : usize,
	from_col : usize,
	to_row : usize,
	to_col : usize
}

impl SimpleMove {
	pub fn new
	(from_row : usize,
			from_column : usize,
			to_row : usize,
			to_column : usize)
	-> SimpleMove{
		SimpleMove{
			from_row : from_row,
			from_col : from_column,
			to_row : to_row,
			to_col : to_column}
	}
	
	pub fn to_row(&self) -> usize {
		self.to_row
	}
	
	pub fn to_column(&self) -> usize {
		self.to_col
	}
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct JumpMove {
	from_row : usize,
	from_col : usize,
	jumps : Vec<JumpMove>
}

impl JumpMove {
	fn new(from_row : usize, from_col : usize) -> JumpMove {
		JumpMove{ from_row : from_row, from_col : from_col, jumps : Vec::new() }
	}
	
	fn with_jumps(from_row : usize, from_col : usize, jumps : Vec<JumpMove>) -> JumpMove {
		JumpMove{ from_row : from_row, from_col : from_col, jumps : jumps }
	}
}

/// Given the position of a main piece on a board, and the
/// direction this man piece is moving, determines the simple
/// moves available to this piece.
pub fn find_simple_moves_for_man
(board : &Board,
		direction : Direction,
		row : usize,
		col : usize)
-> Vec<SimpleMove> {
	let row_offset = match direction {
		Direction::DecreasingRank => TileOffset::Negative(1),
		Direction::IncreasingRank => TileOffset::Positive(1),
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
(board : &Board,
		player : &Player,
		direction : Direction,
		row : usize,
		col : usize)
-> JumpMove {
	let mut jump_root = JumpMove::new(row, col);

	let (pwnd_row_offset, jump_row_offset) = get_row_offsets(direction);

	find_jump_moves_for_man_rustcursive(
		board, player, &pwnd_row_offset, &jump_row_offset, &mut jump_root);

	jump_root
}

fn find_jump_moves_for_man_rustcursive
(board : &Board,
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
(board : &Board,
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

	let mut the_move = JumpMove::new(offset_row, offset_col);

	find_jump_moves_for_man_rustcursive(
		board, player, &pwnd_row_offset, &jump_row_offset, &mut the_move);

	jumps.jumps.push(the_move);
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct BoardPosition {
	row : usize,
	column : usize
}

impl BoardPosition {
	fn new(row : usize, column : usize) -> BoardPosition {
		BoardPosition{row : row, column : column}
	}
}

pub fn find_jump_moves_for_king
(board : &Board,
		player : &Player,
		row : usize,
		col : usize)
-> JumpMove {
	let mut jump_root = JumpMove::new(row, col);
	let mut jumped_tiles = HashSet::new();

	find_jump_moves_for_king_rustcursive(
		board, player, BoardPosition::new(row, col), &mut jump_root, &mut jumped_tiles);

	jump_root
}

fn find_jump_moves_for_king_rustcursive
(board : &Board,
		player : &Player,
		init_position : BoardPosition,
		curr_jump_root : &mut JumpMove,
		jumped_tiles : &mut HashSet<BoardPosition>) {
	push_jump_for_king_if_valid(
		board,
		player,
		init_position,
		curr_jump_root,
		jumped_tiles,
		TileOffset::Negative(1),
		TileOffset::Negative(2),
		TileOffset::Negative(1),
		TileOffset::Negative(2));

	push_jump_for_king_if_valid(
		board,
		player,
		init_position,
		curr_jump_root,
		jumped_tiles,
		TileOffset::Negative(1),
		TileOffset::Negative(2),
		TileOffset::Positive(1),
		TileOffset::Positive(2));

	push_jump_for_king_if_valid(
		board,
		player,
		init_position,
		curr_jump_root,
		jumped_tiles,
		TileOffset::Positive(1),
		TileOffset::Positive(2),
		TileOffset::Negative(1),
		TileOffset::Negative(2));

	push_jump_for_king_if_valid(
		board,
		player,
		init_position,
		curr_jump_root,
		jumped_tiles,
		TileOffset::Positive(1),
		TileOffset::Positive(2),
		TileOffset::Positive(1),
		TileOffset::Positive(2));
}

fn push_jump_for_king_if_valid
(board : &Board,
		player : &Player,
		init_position : BoardPosition,
		curr_jump_root : &mut JumpMove,
		jumped_tiles : &mut HashSet<BoardPosition>,
		pwnd_row_offset : TileOffset,
		jump_row_offset : TileOffset,
		pwnd_col_offset : TileOffset,
		jump_col_offset : TileOffset) {
	let start_row = curr_jump_root.from_row;
	let start_col = curr_jump_root.from_col;

	let tile_on_board = is_tile_offset_in_bounds(
		board, start_row, start_col, &jump_row_offset, &jump_col_offset);
	if !tile_on_board {
		return;
	}

	let (jumped_row, jumped_col)
		= offset_tile(start_row, start_col, &pwnd_row_offset, &pwnd_col_offset);
	let pwnd_tile = board.get_tile(jumped_row, jumped_col);

	let (end_row, end_col)
		= offset_tile(start_row, start_col, &jump_row_offset, &jump_col_offset);
	let end_tile = board.get_tile(end_row, end_col);

	let tile_blocked = end_tile.get_piece().is_some();

	// The initial position of the jumping piece is OK to jump back to. This is because
	// the jumping piece "floats" around the board while the other pieces remain fixed.
	let at_initial_position = init_position == BoardPosition::new(end_row, end_col);
	if tile_blocked && !at_initial_position {
		return;
	}

	let pwnd_piece_enemy = pwnd_tile
		.get_piece()
		.map(|piece| piece.get_player_id() != player.id)
		.unwrap_or(false);

	if !pwnd_piece_enemy {
		return;
	}

	// check to see if we have already jumped the tile
	let jumped_position = BoardPosition::new(jumped_row, jumped_col);
	if jumped_tiles.contains(&jumped_position) {
		return;
	}

	let mut jump = JumpMove::new(end_row, end_col);

	jumped_tiles.insert(jumped_position);

	find_jump_moves_for_king_rustcursive(board, player, init_position, &mut jump, jumped_tiles);

	jumped_tiles.remove(&jumped_position);

	curr_jump_root.jumps.push(jump);
}

fn get_row_offsets(direction : Direction) -> (TileOffset, TileOffset) {
	let (pwnd_row_offset, jump_row_offset) = match direction {
		Direction::DecreasingRank =>
			(TileOffset::Negative(1), TileOffset::Negative(2)),
		Direction::IncreasingRank =>
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
(board : &Board,
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
(board : &Board,
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
			let the_move = SimpleMove::new(
				start_row, start_col, offset_row, offset_col);
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
(board : &Board,
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
		Direction::DecreasingRank, 0, 4, Vec::new()),

	no_moves_when_max_rank_and_increasing_rank(
		Direction::IncreasingRank, 7, 4, Vec::new()),

	single_move_when_min_file(
		Direction::IncreasingRank, 4, 0, vec![SimpleMove::new(4, 0, 5, 1)]),

	single_move_when_max_file(
		Direction::DecreasingRank, 3, 7, vec![SimpleMove::new(3, 7, 2, 6)]),

	two_moves_when_middle_of_board_1(
		Direction::IncreasingRank,
		3,
		5,
		vec![SimpleMove::new(3, 5, 4, 4), SimpleMove::new(3, 5, 4, 6)]),

	two_moves_when_middle_of_board_2(
		Direction::DecreasingRank,
		1,
		2,
		vec![SimpleMove::new(1, 2, 0, 1), SimpleMove::new(1, 2, 0, 3)])
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
		3, 3, Direction::DecreasingRank, 4, 4, vec![SimpleMove::new(4, 4, 3, 5)]),

	move_blocked_when_tile_occupied_2(
		3, 5, Direction::DecreasingRank, 4, 4, vec![SimpleMove::new(4, 4, 3, 3)])
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
		7, 0, vec![SimpleMove::new(7, 0, 6, 1)]),

	single_move_when_min_rank_and_max_file(
		7, 7, vec![SimpleMove::new(7, 7, 6, 6)]),

	single_move_when_max_rank_and_min_file(
		0, 0, vec![SimpleMove::new(0, 0, 1, 1)]),

	single_move_when_max_rank_and_max_file(
		0, 7, vec![SimpleMove::new(0, 7, 1, 6)]),

	four_moves_when_middle_of_board(
		3, 5, vec![
			SimpleMove::new(3, 5, 2, 4),
			SimpleMove::new(3, 5, 2, 6),
			SimpleMove::new(3, 5, 4, 4),
			SimpleMove::new(3, 5, 4, 6)])
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
			SimpleMove::new(3, 5, 2, 6),
			SimpleMove::new(3, 5, 4, 4),
			SimpleMove::new(3, 5, 4, 6)]),

	move_blocked_when_tile_occupied_2(
		2, 6, 3, 5, vec![
			SimpleMove::new(3, 5, 2, 4),
			SimpleMove::new(3, 5, 4, 4),
			SimpleMove::new(3, 5, 4, 6)]),

	move_blocked_when_tile_occupied_3(
		4, 4, 3, 5, vec![
			SimpleMove::new(3, 5, 2, 4),
			SimpleMove::new(3, 5, 2, 6),
			SimpleMove::new(3, 5, 4, 6)]),

	move_blocked_when_tile_occupied_4(
		4, 6, 3, 5, vec![
			SimpleMove::new(3, 5, 2, 4),
			SimpleMove::new(3, 5, 2, 6),
			SimpleMove::new(3, 5, 4, 4)])
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
	let direction = Direction::IncreasingRank;

	let result = find_jump_moves_for_man(
		&board, &player, direction, start_row, start_col);

	let exp_result = JumpMove::new(start_row, start_col);

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
	let direction = Direction::IncreasingRank;

	let opponent = Player{ id : 1 };

	let enemy_piece = ManPiece::new(&opponent);
	let enemy_tile = OccupiedTile::new(Box::new(enemy_piece));
	board.set_tile(enemy_row, enemy_col, Box::new(enemy_tile));

	let result = find_jump_moves_for_man(
		&board, &player, direction, start_row, start_col);

	assert_eq!(exp_result, result);
}

ptest!(test_single_jump_single_enemy [
	jumping_adjacent_enemy_left(
	4, 3, 5, 2, JumpMove::with_jumps(4, 3, vec![JumpMove::new(6, 1)])),

	jumping_adjacent_enemy_right(
		4, 3, 5, 4, JumpMove::with_jumps(4, 3, vec![JumpMove::new(6, 5)]))
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
	let direction = Direction::IncreasingRank;

	let opponent = Player{ id : 1 };

	let left_enemy_piece = ManPiece::new(&opponent);
	let left_enemy_tile = OccupiedTile::new(Box::new(left_enemy_piece));
	board.set_tile(left_enemy_row, left_enemy_col, Box::new(left_enemy_tile));

	let right_enemy_piece = ManPiece::new(&opponent);
	let right_enemy_tile = OccupiedTile::new(Box::new(right_enemy_piece));
	board.set_tile(right_enemy_row, right_enemy_col, Box::new(right_enemy_tile));

	let result = find_jump_moves_for_man(
		&board, &player, direction, start_row, start_col);

	assert_eq!(exp_result, result);
}
	
ptest!(test_single_jump_two_enemies [
	jumping_two_forward_adjacent_enemies(
		4, 3, 5, 2, 5, 4, JumpMove::with_jumps(4, 3, vec![JumpMove::new(6, 1), JumpMove::new(6, 5)])),

	jumping_two_backward_adjacent_enemies(
		6, 3, 5, 2, 5, 4, JumpMove::new(6, 3))
]);

fn test_jumping_friendly_piece
(start_row : usize,
		start_col : usize,
		friendly_row : usize,
		friendly_col : usize) {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::IncreasingRank;

	let left_piece = ManPiece::new(&player);
	let left_tile = OccupiedTile::new(Box::new(left_piece));
	board.set_tile(friendly_row, friendly_col, Box::new(left_tile));

	let result = find_jump_moves_for_man(
		&board, &player, direction, start_row, start_col);

	let exp_result = JumpMove::new(start_row, start_col);

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
	let direction = Direction::IncreasingRank;

	let opponent = Player{ id : 1 };
	let pwnd_piece = ManPiece::new(&opponent);
	let pwnd_tile = OccupiedTile::new(Box::new(pwnd_piece));
	board.set_tile(pwnd_row, pwnd_col, Box::new(pwnd_tile));

	let block_piece = ManPiece::new(&player);
	let block_tile = OccupiedTile::new(Box::new(block_piece));
	board.set_tile(blocked_row, blocked_col, Box::new(block_tile));

	let result = find_jump_moves_for_man(
		&board, &player, direction, start_row, start_col);

	let exp_result = JumpMove::new(start_row, start_col);

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
	let direction = Direction::IncreasingRank;
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
		&board, &player, direction, start_row, start_col);

	let exp_result = JumpMove::with_jumps(
		start_row, start_col, vec![JumpMove::new(6, 5)]);

	assert_eq!(exp_result, result);
}

#[test]
fn jumping_two_forward_adjacent_enemies_right_blocked() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::IncreasingRank;
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
		&board, &player, direction, start_row, start_col);

	let exp_result = JumpMove::with_jumps(
		start_row, start_col, vec![JumpMove::new(6, 1)]);

	assert_eq!(exp_result, result);
}

#[test]
fn the_one_true_test() {
	let mut board = Board::new(8, 8);
	let player = Player{ id : 0 };
	let direction = Direction::DecreasingRank;

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
		&board, &player, direction, start_row, start_col);

	let exp_result = JumpMove::with_jumps(
		start_row,
		start_col,
		vec![JumpMove::with_jumps(
			4,
			1,
			vec![
				JumpMove::with_jumps(
					2,
					3,
					vec![JumpMove::new(0, 1), JumpMove::new(0, 5)])]),
			JumpMove::with_jumps(
				4,
				5,
				vec![
					JumpMove::with_jumps(
						2,
						3,
						vec![JumpMove::new(0, 1), JumpMove::new(0, 5)]),
					JumpMove::new(2, 7)])]);

	assert_eq!(exp_result, result);
}

}

mod king_piece {

use super::super::super::*;
use checkers::Board;
use checkers::ManPiece;
use checkers::OccupiedTile;
use checkers::Player;

#[test]
fn no_adjacent_enemy_pieces() {
	let board = Board::new(8, 8);
	let player = Player{ id : 0 };

	let start_row = 6;
	let start_col = 3;

	let result = find_jump_moves_for_king(
		&board, &player, start_row, start_col);
	let exp_result = JumpMove::new(start_row, start_col);

	assert_eq!(exp_result, result);
}

fn test_single_jump
(enemy_row : usize, enemy_col : usize, exp_result : JumpMove) {
	let mut board = Board::new(8, 8);

	let player = Player{ id : 0 };
	let opponent = Player{ id : 1 };

	let enemy_piece = ManPiece::new(&opponent);
	let enemy_tile = OccupiedTile::new(Box::new(enemy_piece));
	board.set_tile(enemy_row, enemy_col, Box::new(enemy_tile));

	let result = find_jump_moves_for_king(
		&board, &player, 4, 3);

	assert_eq!(exp_result, result);
}

ptest!(test_single_jump [
	single_jump_decr_rank_decr_file(5, 2, JumpMove::with_jumps(4, 3, vec![JumpMove::new(6, 1)])),
	single_jump_decr_rank_incr_file(5, 4, JumpMove::with_jumps(4, 3, vec![JumpMove::new(6, 5)])),
	single_jump_incr_rank_decr_file(3, 4, JumpMove::with_jumps(4, 3, vec![JumpMove::new(2, 5)])),
	single_jump_incr_rank_incr_file(3, 2, JumpMove::with_jumps(4, 3, vec![JumpMove::new(2, 1)]))
]);

#[test]
fn jump_multiple_directions() {
	let mut board = Board::new(8, 8);

	let player = Player{ id : 0 };
	let opponent = Player{ id : 1 };

	let enemy_piece1 = ManPiece::new(&opponent);
	let enemy_tile1 = OccupiedTile::new(Box::new(enemy_piece1));
	board.set_tile(3, 2, Box::new(enemy_tile1));

	let enemy_piece2 = ManPiece::new(&opponent);
	let enemy_tile2 = OccupiedTile::new(Box::new(enemy_piece2));
	board.set_tile(3, 4, Box::new(enemy_tile2));

	let enemy_piece3 = ManPiece::new(&opponent);
	let enemy_tile3 = OccupiedTile::new(Box::new(enemy_piece3));
	board.set_tile(5, 4, Box::new(enemy_tile3));

	let enemy_piece4 = ManPiece::new(&opponent);
	let enemy_tile4 = OccupiedTile::new(Box::new(enemy_piece4));
	board.set_tile(5, 2, Box::new(enemy_tile4));

	let start_row = 4;
	let start_col = 3;

	let result = find_jump_moves_for_king(
		&board, &player, start_row, start_col);
	let exp_result = JumpMove::with_jumps(
		start_row,
		start_col,
		vec![
			JumpMove::new(2, 1),
			JumpMove::new(2, 5),
			JumpMove::new(6, 1),
			JumpMove::new(6, 5) ] );

	assert_eq!(exp_result, result);
}

#[test]
fn jump_in_a_circle() {
	let mut board = Board::new(8, 8);

	let player = Player{ id : 0 };
	let opponent = Player{ id : 1 };

	let start_row = 4;
	let start_col = 1;

	let friendly_piece = ManPiece::new(&player);
	let friendly_tile = OccupiedTile::new(Box::new(friendly_piece));
	board.set_tile(start_row, start_col, Box::new(friendly_tile));

	let enemy_piece1 = ManPiece::new(&opponent);
	let enemy_tile1 = OccupiedTile::new(Box::new(enemy_piece1));
	board.set_tile(3, 2, Box::new(enemy_tile1));

	let enemy_piece2 = ManPiece::new(&opponent);
	let enemy_tile2 = OccupiedTile::new(Box::new(enemy_piece2));
	board.set_tile(3, 4, Box::new(enemy_tile2));

	let enemy_piece3 = ManPiece::new(&opponent);
	let enemy_tile3 = OccupiedTile::new(Box::new(enemy_piece3));
	board.set_tile(5, 4, Box::new(enemy_tile3));

	let enemy_piece4 = ManPiece::new(&opponent);
	let enemy_tile4 = OccupiedTile::new(Box::new(enemy_piece4));
	board.set_tile(5, 2, Box::new(enemy_tile4));

	let result = find_jump_moves_for_king(
		&board, &player, start_row, start_col);
	let exp_result = JumpMove::with_jumps(
		start_row,
		start_col,
		vec![
			JumpMove::with_jumps(2, 3, 
				vec![JumpMove::with_jumps(4, 5,
					vec![JumpMove::with_jumps(6, 3,
						vec![JumpMove::new(start_row, start_col)])])]),			
			JumpMove::with_jumps(6, 3, 
				vec![JumpMove::with_jumps(4, 5,
					vec![JumpMove::with_jumps(2, 3,
						vec![JumpMove::new(start_row, start_col)])])])]);	

	assert_eq!(exp_result, result);
}

}

}

}
