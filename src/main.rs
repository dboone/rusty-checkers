mod checkers;
use checkers::Board;
use checkers::Player;

fn main() {
    println!("Welcome to Draughts!");

	let player1 = Player{ id : 0 };
	let player2 = Player{ id : 1 };
    let board = Board::new_checkerboard(&player1, &player2);
    let tile = board.get_tile(0, 0);    
}
