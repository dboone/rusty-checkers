mod checkers;
use checkers::Board;

fn main() {
    println!("Welcome to Draughts!");

    let board = Board::new_checkerboard();
    let tile = board.get_tile(0, 0);    
}
