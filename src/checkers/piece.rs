use checkers::player::Player;

pub trait Piece {
	fn get_player_id(&self) -> u32;
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
}
