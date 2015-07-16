use checkers::ai;
use checkers::Board;
use checkers::Direction;
use checkers::PieceType;
use checkers::Player;
use checkers::SimpleMove;

pub enum GameState {
	InProgress,
	GameOver
}

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
	
	pub fn board(&self) -> &Board {
		&self.board
	}
	
	pub fn apply_simple_move(&self, the_move : SimpleMove) -> Result<GameState, MoveError> {
		let simple_moves = self.find_available_simple_moves();
	
		Ok(GameState::InProgress)
	}
	
	//TODO
	// - receive jump move
	// - compute available moves
	//   - compute jump moves
	// - check that player's move is one of the available moves
	// - apply player's move
	//   - move chosen piece
	//   - remove jumped pieces
	//   - king man pieces that reach other side
	// - swap current player
	// - check if game is over
}