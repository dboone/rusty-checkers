use checkers::ai;

use checkers::Board;
use checkers::BoardPosition;
use checkers::Direction;
use checkers::JumpMove;
use checkers::KingPiece;
use checkers::ManPiece;
use checkers::OccupiedTile;
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
	
	current_player_index : usize,
	
	available_simple_moves : Vec<SimpleMove>,
	available_jump_moves : Vec<JumpMove>
}

impl Game {
	pub fn new() -> Game {
		let (player1, player2) = Game::two_players();
		
		let board = Board::new_checkerboard(&player1, &player2);
		
		Game::with_board_and_players(board, player1, player2)
	}
	
	/// Creates a new Checkers game with an 8x8 board and the specified piece
	/// layout `player1_positions` contains the pieces for the first player,
	/// and `player2_positions` contains the pieces for the second player. All
	/// pieces are initially `ManPiece`s.
	///
	/// # Panics
	///
	/// Panics if any element in *player1_positions* or *player2_positions*
	/// is a `BoardPosition` with a row or column outside the range [0, 7].
	/// Also panics if any two positions are exactly the same.
	#[cfg(test)]
	pub fn with_piece_positions
	(player1_positions : Vec<BoardPosition>,
			player2_positions : Vec<BoardPosition>)
	-> Game {
		let CHECKERBOARD_SIZE : usize = 8;
		let mut board = Board::new(CHECKERBOARD_SIZE, CHECKERBOARD_SIZE);
		
		let (player1, player2) = Game::two_players();
		
		Game::initialize_pieces(&mut board, &player1, &player1_positions);
		Game::initialize_pieces(&mut board, &player2, &player2_positions);
		
		Game::with_board_and_players(board, player1, player2)
	}
	
	// creates and returns two players with distinct IDs
	fn two_players() -> (Player, Player) {
		(Player{id : 1}, Player{id : 2})
	}
	
	// adds man pieces belonging to a particular player
	// at the specified positions on a board
	fn initialize_pieces
	(board : &mut Board, player : &Player, positions : &Vec<BoardPosition>) {
		for pos in positions {
			let piece = ManPiece::new(&player);
			let tile = OccupiedTile::new(Box::new(piece));
			assert!(board.get_tile(pos.row, pos.column).get_piece().is_none());
			board.set_tile(pos.row, pos.column, Box::new(tile));
		}
	}
	
	fn with_board_and_players
	(board : Board, player1 : Player, player2 : Player)
	-> Game {
		let player1_info = PlayerInfo{
			player : player1, direction : Direction::IncreasingRank};
		let player2_info = PlayerInfo{
			player : player2, direction : Direction::DecreasingRank};
		
		let mut game = Game{
			players : [player1_info, player2_info],
			board : board,
			current_player_index : 0,
			available_simple_moves : Vec::new(),
			available_jump_moves : Vec::new()};
			
		game.find_available_moves();
		
		game
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
	
	fn find_available_moves(&mut self) {
		self.available_simple_moves = self.find_available_simple_moves();
		self.available_jump_moves = self.find_available_jump_moves();
	}
	
	pub fn board(&self) -> &Board {
		&self.board
	}
	
	pub fn current_player(&self) -> &Player {
		&self.current_player_info().player
	}
	
	fn check_for_coronation
	(&mut self, row : usize, col : usize) {
		let coronate = match self.board.get_tile(row, col).get_piece() {
			Some(piece) =>
				match piece.get_type() {
					PieceType::Man =>
						match self.current_player_info().direction {
							Direction::IncreasingRank =>
								row + 1 == self.board.number_rows(),
							Direction::DecreasingRank => row == 0
						},
					PieceType::King => false
				},
			None => unreachable!()
		};
		
		if coronate {
			let king = KingPiece::new(&self.current_player_info().player);
			let tile = OccupiedTile::new(Box::new(king));
			self.board.set_tile(row, col, Box::new(tile))
		}
	}
	
	fn select_next_player(&mut self) {
		self.current_player_index = 1 - self.current_player_index;
	}
	
	fn is_game_over(&self) -> bool {
		self.available_simple_moves.is_empty()
			&& self.available_jump_moves.is_empty()
	}
	
	fn finish_move
	(&mut self, final_row : usize, final_col : usize)
	-> GameState {
		self.check_for_coronation(final_row, final_col);
		
		self.select_next_player();
		self.find_available_moves();
		
		if self.is_game_over() {
			GameState::GameOver
		} else {
			GameState::InProgress
		}
	}
	
	pub fn apply_simple_move(&mut self, the_move : SimpleMove) -> Result<GameState, MoveError> {
		if self.available_jump_moves.is_empty() {
			if self.available_simple_moves.contains(&the_move) {
				self.board.swap_tiles(
					the_move.from_row(),
					the_move.from_column(),
					the_move.to_row(),
					the_move.to_column());
				
				let game_state = self.finish_move(
					the_move.to_row(), the_move.to_column());
				Ok(game_state)
			} else {
				Err(MoveError::InvalidMove)
			}
		} else {
			Err(MoveError::ShouldHaveJumped)
		}
	}
	
	pub fn apply_jump_move(&mut self, the_move : Vec<BoardPosition>) -> Result<GameState, MoveError> {
		let jump_valid = self.available_jump_moves.iter()
			.any(|jump_tree| jump_tree.contains_jump_sequence(&the_move));
		if jump_valid {
			let start_position = the_move.first().unwrap();
			let final_position = the_move.last().unwrap();
			
			// move the jumping piece
			self.board.swap_tiles(
				start_position.row,
				start_position.column,
				final_position.row,
				final_position.column);
			
			// remove all jumped pieces
			let iter = the_move[0..].iter().zip(the_move[1..].iter());
			for (jump_from_pos, jump_to_pos) in iter {
				let jumped_row = jump_from_pos.row + jump_to_pos.row / 2;
				let jumped_col = jump_from_pos.column + jump_to_pos.column / 2;
				self.board.clear_tile(jumped_row, jumped_col);
			}

			let game_state = self.finish_move(
				final_position.row, final_position.column);
			Ok(game_state)
		} else {
			Err(MoveError::InvalidMove)
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	use checkers::BoardPosition;
	use checkers::SimpleMove;

	#[test]
	fn test_good_simple_move() {
		let mut game = Game::new();
		let result = game.apply_simple_move(SimpleMove::new(2, 0, 3, 1));
		let exp_result : Result<GameState, MoveError> = Ok(GameState::InProgress);
		assert_eq!(exp_result, result);
		
		let player_id = game.current_player().id;
		assert_eq!(2, player_id);
		
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
		
		let player_id = game.current_player().id;
		assert_eq!(1, player_id);
		
		//TODO this test should be more thorough (e.g. check the piece
		// type, player ID, etc.), but it's good enough for now
		assert!(game.board().get_tile(3, 0).get_piece().is_none());
	}
	
	#[test]
	fn test_bad_jump_move() {
		let mut game = Game::new();
		let result = game.apply_jump_move(
			vec![BoardPosition::new(2, 0), BoardPosition::new(4, 2)]);
		let exp_result : Result<GameState, MoveError> = Err(MoveError::InvalidMove);
		assert_eq!(exp_result, result);
		
		let player_id = game.current_player().id;
		assert_eq!(1, player_id);
		
		//TODO these tests should be more thorough (e.g. check the piece
		// type, player ID, etc.), but it's good enough for now
		assert!(game.board().get_tile(2, 0).get_piece().is_some());
		assert!(game.board().get_tile(4, 2).get_piece().is_none());
	}
	
	//TODO test applying a simple move when a jump is available
	
	//TODO test that coronation works
	
	//TODO test that game over is correctly detected
	
	//TODO test applying a good jump move
}