use std::ops::Deref;

pub trait Piece {
}

pub struct ManPiece;

impl Piece for ManPiece {
}

pub struct KingPiece;

impl Piece for KingPiece {
}

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

impl Tile for OccupiedTile {
    fn get_piece(&self) -> Option<&Piece> {
       Option::Some(self.piece.deref())
    }
}

pub struct Board {
    number_rows : usize,
    number_columns : usize,
    tiles : Vec<Box<Tile>>
}

const CHECKERBOARD_SIZE : usize = 8;
const CHECKERS_NUMBER_TILES : usize = CHECKERBOARD_SIZE * CHECKERBOARD_SIZE;

impl Board {
    fn new() -> Board {
        let mut board = Board {
            number_rows : CHECKERBOARD_SIZE,
            number_columns : CHECKERBOARD_SIZE,
            tiles : Vec::with_capacity(CHECKERS_NUMBER_TILES)
        };

        for _ in 0..CHECKERS_NUMBER_TILES {
            board.tiles.push(Box::new(EmptyTile));
        }

        board
    }

    fn get_tile(&self, row : usize, column : usize) -> &Tile {
        self.tiles[ row + self.number_rows * column ].deref()
    }
}

fn main() {
    println!("Welcome to Draughts!");

    let board = Board::new();
    let tile = board.get_tile(0, 0);    
}
