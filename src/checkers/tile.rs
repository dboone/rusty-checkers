use std::ops::Deref;
use checkers::piece::Piece;

pub trait Tile {
    fn get_piece(&self) -> Option<&Piece>;
}

pub struct EmptyTile;

impl Tile for EmptyTile {
    fn get_piece(&self) -> Option<&Piece> {
       Option::None
    }
}

pub struct OccupiedTile {
    piece : Box<Piece>
}

impl OccupiedTile {
    pub fn new( piece : Box<Piece> ) -> OccupiedTile {
        OccupiedTile {
            piece : piece
        } 
    }
}

impl Tile for OccupiedTile {
    fn get_piece(&self) -> Option<&Piece> {
       Option::Some(self.piece.deref())
    }
}
