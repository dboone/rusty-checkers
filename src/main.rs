use std::ops::Deref;

pub trait Tile {
}

pub struct EmptyTile;

impl Tile for EmptyTile {
    // Tile functions
}

pub struct Board {
    number_rows : usize,
    number_columns : usize,
    tiles : Vec<Box<Tile>>
}

const CHECKERS_BOARD_SIZE : usize = 8;
const CHECKERS_NUMBER_TILES : usize = CHECKERS_BOARD_SIZE * CHECKERS_BOARD_SIZE;

impl Board {
    fn new() -> Board {
       let mut board = Board {
           number_rows : CHECKERS_BOARD_SIZE,
	   number_columns : CHECKERS_BOARD_SIZE,
	   tiles : Vec::with_capacity(CHECKERS_NUMBER_TILES)
       };

       // initialize tiles
       for _ in 0..CHECKERS_NUMBER_TILES {
           board.tiles.push(Box::new(EmptyTile));
       }

       board
    }

    fn get_tile(&self, row: usize, column: usize) -> &Tile {
       self.tiles[ row + self.number_rows * column ].deref()
    }
}

fn main() {
    println!("Welcome to Draughts!");

    let board = Board::new();
    let tile = board.get_tile(0, 0);    
}
