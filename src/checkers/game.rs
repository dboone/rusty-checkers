use checkers::ai;

use checkers::Board;
use checkers::Direction;
use checkers::JumpMove;
use checkers::PieceType;
use checkers::Player;
use checkers::SimpleMove;

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
	InProgress,
	GameOver
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
	InvalidMove,
	ShouldHaveJumped
}

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
	
	fn current_player_info(&self) -> &PlayerInfo {
		&self.players[self.current_player_index]
	}
	
	fn find_available_simple_moves(&self) -> Vec<SimpleMove> {
		let mut moves = Vec::new();
	
		let curr_player_info = self.current_player_info();
		let curr_player = &curr_player_info.player;
		let curr_player_id = curr_player.id;
		let curr_direction = curr_player_info.direction;
		for r in 0..self.board.number_rows() {
			for c in 0..self.board.number_columns() {
				match self.board.get_tile(r, c).get_piece() {
					Some(piece) =>
						if piece.get_player_id() == curr_player_id {
							let piece_moves = match piece.get_type() {
								PieceType::Man =>
									ai::find_simple_moves_for_man(
										&self.board, curr_direction, r, c),
								PieceType::King =>
									ai::find_simple_moves_for_king(
										&self.board, r, c),
							};
							moves.extend(piece_moves);
						},
					None => {}
				}
			}
		}
		
		moves
	}
	
	fn find_available_jump_moves(&self) -> Vec<JumpMove> {
		let mut moves = Vec::new();
	
		let curr_player_info = self.current_player_info();
		let curr_player = &curr_player_info.player;
		let curr_player_id = curr_player.id;
		let curr_direction = curr_player_info.direction;
		for r in 0..self.board.number_rows() {
			for c in 0..self.board.number_columns() {
				match self.board.get_tile(r, c).get_piece() {
					Some(piece) =>
						if piece.get_player_id() == curr_player_id {
							let jump_move = match piece.get_type() {
								PieceType::Man =>
									ai::find_jump_moves_for_man(
										&self.board, curr_player, curr_direction, r, c),
								PieceType::King =>
									ai::find_jump_moves_for_king(
										&self.board, curr_player, r, c),
							};
							if !jump_move.jumps().is_empty() {
								moves.push(jump_move);
							}
						},
					None => {}
				}
			}
		}
		
		moves
	}
	
	pub fn board(&self) -> &Board {
		&self.board
	}
	
	pub fn apply_simple_move(&mut self, the_move : SimpleMove) -> Result<GameState, MoveError> {
		let jump_moves = self.find_available_jump_moves();
		if jump_moves.is_empty() {
			let simple_moves = self.find_available_simple_moves();
			if simple_moves.contains(&the_move) {
				self.board.swap_tiles(
					the_move.from_row(),
					the_move.from_column(),
					the_move.to_row(),
					the_move.to_column());
				Ok(GameState::InProgress)
			} else {
				Err(MoveError::InvalidMove)
			}
		} else {
			Err(MoveError::ShouldHaveJumped)
		}
	}
	
	//TODO
	// - receive jump move
	// - check that player's move is one of the available moves
	// - apply player's move
	//   - move chosen piece
	//   - remove jumped pieces
	//   - king man pieces that reach other side
	// - swap current player
	// - check if game is over
}

#[cfg(test)]
mod test {
	use super::*;
	
	use checkers::SimpleMove;

	#[test]
	fn test_good_simple_move() {
		let mut game = Game::new();
		let result = game.apply_simple_move(SimpleMove::new(2, 0, 3, 1));
		let exp_result : Result<GameState, MoveError> = Ok(GameState::InProgress);
		assert_eq!(exp_result, result);
		
		//TODO this test should be more thorough (e.g. check the piece
		// type, player ID, etc.), but it's good enough for now
		assert!(game.board().get_tile(3, 1).get_piece().is_some());
	}
	
	#[test]
	fn test_bad_simple_move() {
		let mut game = Game::new();
		let result = game.apply_simple_move(SimpleMove::new(2, 0, 3, 0));
		let exp_result : Result<GameState, MoveError> = Err(MoveError::InvalidMove);
		assert_eq!(exp_result, result);
		
		//TODO this test should be more thorough (e.g. check the piece
		// type, player ID, etc.), but it's good enough for now
		assert!(game.board().get_tile(3, 0).get_piece().is_none());
	}
	
	//TODO test applying a simple move when a jump is available
}