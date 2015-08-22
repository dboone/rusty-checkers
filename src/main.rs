#[macro_use]
extern crate corrosion;
use std::io::stdout;

mod checkers;
use checkers::Game;

fn main() {
    println!("Welcome to Draughts!");

	let game = Game::new();
	
	let mut writer = stdout();
	checkers::display::print_board(&mut writer, game.board()).unwrap();
}
