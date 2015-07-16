use checkers::Board;
use checkers::Direction;
use checkers::Player;

use std::collections::HashMap;

struct PlayerInfo {
	player : Player,
	direction : Direction
}

pub struct Game {
	players : [PlayerInfo; 2],
	board : Board,
	
	current_player_index : usize
}

impl Game {
	pub fn new() -> Game {
		let player1 = Player{id : 1};
		let player2 = Player{id : 2};
		
		let board = Board::new_checkerboard(&player1, &player2);
		
		let player1_info = PlayerInfo{
			player : player1, direction : Direction::IncreasingRank};
		let player2_info = PlayerInfo{
			player : player2, direction : Direction::DecreasingRank};
		
		Game{
			players : [player1_info, player2_info],
			board : board,
			current_player_index : 0}
	}
	
	pub fn board(&self) -> &Board {
		&self.board
	}
	
	//TODO
	// - receive player's move
	// - compute available moves
	// - check that player's move is one of available moves
	// - apply player's move
	//   - move chosen piece
	//   - remove jumped pieces
	//   - king pieces that reach other side
	// - swap current player
}