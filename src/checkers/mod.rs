mod player;
pub use checkers::player::Player;

mod piece;
pub use checkers::piece::*;

mod tile;
pub use checkers::tile::*;

pub struct Board {
    number_rows : usize,
    number_columns : usize,
    tiles : Vec<Box<Tile>>
}

const CHECKERBOARD_SIZE : usize = 8;
const CHECKERS_NUMBER_TILES : usize = CHECKERBOARD_SIZE * CHECKERBOARD_SIZE;

impl Board {
    pub fn new() -> Board {
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

    pub fn new_checkerboard(player1 : &Player, player2 : &Player) -> Board {
		if player1.id == player2.id {
			panic!("Player 1 and Player 2 have the same ID: {}", player1.id)
		}
	
        let mut board = Board {
            number_rows : CHECKERBOARD_SIZE,
            number_columns : CHECKERBOARD_SIZE,    	   
            tiles : Vec::with_capacity(CHECKERS_NUMBER_TILES)
        };

        Board::fill_odd_row(&mut board, player1);
        Board::fill_even_row(&mut board, player1);
        Board::fill_odd_row(&mut board, player1);

        Board::fill_empty_row(&mut board);
        Board::fill_empty_row(&mut board);

        Board::fill_even_row(&mut board, player2);
        Board::fill_odd_row(&mut board, player2);
        Board::fill_even_row(&mut board, player2);

        board
    }

	fn indices_to_index(&self, row : usize, column : usize) -> usize {
        row + self.number_rows * column
	}
	
    pub fn get_tile(&self, row : usize, column : usize) -> &Tile {
        let idx = self.indices_to_index(row, column);
        &*self.tiles[idx]
    }

	pub fn set_tile(
			&mut self,
			row : usize,
			column : usize,
			tile : Box<Tile>) {
        let idx = self.indices_to_index(row, column);
        self.tiles[idx] = tile;
	}

	pub fn clear_tile(&mut self, row : usize, column : usize) {
		self.set_tile(row, column, Box::new(EmptyTile));
	}

	pub fn swap_tiles(
			&mut self,
			row1 : usize,
			column1 : usize,
			row2 : usize,
			column2 : usize) {
        let idx1 = self.indices_to_index(row1, column1);
        let idx2 = self.indices_to_index(row2, column2);
		self.tiles.swap(idx1, idx2);
	}

    fn fill_even_row(board : &mut Board, player : &Player) {
        for t in 0..CHECKERBOARD_SIZE {
            let tile : Box<Tile> = if t % 2 == 0 {
				let piece = ManPiece::new(player);
                Box::new(OccupiedTile::new(Box::new(piece)))
            } else {
                Box::new(EmptyTile)
			};
            board.tiles.push(tile);
        }
    }

    fn fill_odd_row(board : &mut Board, player : &Player) {
        for t in 0..CHECKERBOARD_SIZE {
            let tile : Box<Tile> = if t % 2 == 1 {
				let piece = ManPiece::new(player);
                Box::new(OccupiedTile::new(Box::new(piece)))
            } else {
                Box::new(EmptyTile)
            };
            board.tiles.push(tile);
        }
    }

    fn fill_empty_row(board : &mut Board) {
        for t in 0..CHECKERBOARD_SIZE {
            board.tiles.push(Box::new(EmptyTile));
        }
    }
}
