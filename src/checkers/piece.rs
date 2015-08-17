use checkers::player::Player;

pub enum PieceType {
	Man,
	King
}

pub trait Piece {
	fn get_player_id(&self) -> u32;
	
	fn get_type(&self) -> PieceType;
}

pub struct ManPiece {
	player_id : u32
}

impl ManPiece {
	pub fn new(player : &Player) -> ManPiece {
		ManPiece{ player_id : player.id }
	}
}

impl Piece for ManPiece {
	fn get_player_id(&self) -> u32 {
		self.player_id
	}
	
	fn get_type(&self) -> PieceType {
		PieceType::Man
	}
}

pub struct KingPiece {
	player_id : u32
}

impl KingPiece {
	pub fn new(player : &Player) -> KingPiece {
		KingPiece{ player_id : player.id }
	}
}

impl Piece for KingPiece {
	fn get_player_id(&self) -> u32 {
		self.player_id
	}
	
	fn get_type(&self) -> PieceType {
		PieceType::King
	}
}
