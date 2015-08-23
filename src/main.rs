#[macro_use]
extern crate corrosion;

use std::io::{stdin, stdout, Write};

mod checkers;
use checkers::{
	BoardPosition,
	Game,
	GameState,
	InputError,
	MoveError,
	SimpleMove,
	TokenError};

mod util;

fn apply_positions_as_move
(game : &mut Game, positions : Vec<BoardPosition>)
-> Result<GameState, MoveError> {
	if positions.len() == 2 {
		let start = positions[0];
		let end = positions[1];
		
		let row_diff = util::absolute_diff(start.row, end.row);
		let col_diff = util::absolute_diff(start.column, end.column);
		
		if row_diff == 1 && col_diff == 1 {
			game.apply_simple_move(SimpleMove::new(
				start.row, start.column, end.row, end.column))
		} else {
			game.apply_jump_move(positions)
		}
	} else {
		game.apply_jump_move(positions)
	}
}

enum PlayerColor {
	Red,
	Black
}

fn player_id_to_color(player_id : u32) -> PlayerColor {
	match player_id {
		1 => PlayerColor::Red,
		2 => PlayerColor::Black,
		_ => unreachable!()
	}
}

fn player_color_to_name(color : PlayerColor) -> &'static str {
	match color {
		PlayerColor::Red => "Red",
		PlayerColor::Black => "Black"
	}
}

fn main() {
    println!("Welcome to Draughts!");

	let mut writer = stdout();
	let mut game = Game::new();
	let mut line = String::new();
	
	checkers::print_board(&mut writer, game.board()).unwrap();
	
	'game_loop: loop {
		line.clear();
		
		let player_name = player_color_to_name(
			player_id_to_color(
				game.current_player().id));
		
		print!("\n{}'s move: ", player_name);
		
		for _ in stdout().flush().err() {
			panic!("Something really bad happened!");
		}
		
		for _ in stdin().read_line(&mut line).err() {
			panic!("Something really bad happened!");
		}
		
		let line = line.trim();
		
		match line {
			"Q" | "q" => {
				println!("\nGiving up so soon?");
				break 'game_loop;
			}
			_ => { }
		}
		
		let parse_result = checkers::parse_move(&line);
		
		match parse_result {
			Ok(positions) => {
				let move_result = apply_positions_as_move(&mut game, positions);
				match move_result {
					Ok(game_state) => match game_state {
						GameState::InProgress => { },
						GameState::GameOver{winner_id} => {
							let player_name = player_color_to_name(
								player_id_to_color(winner_id));
							
							println!("\nGame over! {} won!", player_name);
							break 'game_loop;
						}
					},
					Err(e) => match e {
						MoveError::InvalidMove => println!("\n *** Illegal move"),
						MoveError::ShouldHaveJumped => println!("\n *** Must take jump")
					}
				}
			},
			Err(e) => match e {
				InputError::TooFewTokens =>
					println!("\n *** You must specify at least two board positions"),
				InputError::InvalidTokens{tokens : errors} => {
					for error in errors {
						match error {
							TokenError::MissingFile{token} =>
								println!("\n *** Board position '{}' must specify file", token),
							TokenError::MissingRank{token} =>
								println!("\n *** Board position '{}' must specify rank", token),
							TokenError::ZeroRank{token} =>
								println!("\n *** Rank cannot be zero: {}", token),
							TokenError::InvalidCharacter{token, char_index} => {
								let ch = token.chars().nth(char_index).unwrap();
								println!("\n *** Board position '{}' contains invalid character '{}'", token, ch);
							}
						}
					}
				}
			}
		}
		
		println!("");
		checkers::print_board(&mut writer, game.board()).unwrap();
	}
}
