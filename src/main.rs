mod view;
mod game;
mod board;
mod piece;
mod command;
#[allow(non_snake_case)]
mod SDL;
mod c_api;

use view::View;
use command::Command;
use SDL::*;

fn main() {

	View::init_graph();
	let screen_height = View::get_screen_height();
	let mut game = game::Game::new(screen_height);

	let mut sdl_event = SDL_Event {
		type_: 0
	};

	let mut time1 = View::sdl_get_tickets();

	let mut quit = false;

	'main_loop:
	while !quit {
		game.draw_scene();

		let key = Command::poll_key(&mut sdl_event);

		while Command::sdl_poll_event(&mut sdl_event) != 0 {
			unsafe {
				if sdl_event.type_ == SDL_QUIT {
					quit = true;
				}
			}
		}

		match key {
			10 => quit = true,
			SDLK_RIGHT => {
				if game.is_possible_movement(1, 0) {
					game.pos_x += 1;
				}
			},
			SDLK_LEFT => {
				if game.is_possible_movement(-1, 0) {
					game.pos_x -= 1;
				}
			},
			SDLK_DOWN => {
				if game.is_possible_movement(0, 1) {
					game.pos_y += 1;
				}
			},
			SDLK_SPACE => {
				while game.is_possible_movement(0, 0) {
					game.pos_y += 1;
				}
				game.board.store_piece(game.pos_x, game.pos_y - 1);
				game.board.delete_possible_lines();

				if game.board.is_game_over() {
					if !game.restart() {
						break 'main_loop;
					}
				}

				game.create_new_piece();
			},
			SDLK_UP => {
				game.board.piece.rotate_piece(1);
				if !game.is_possible_movement(0, 0) {
					game.board.piece.rotate_piece(3);
				}
			}
			_ => {
				// println!("??????????? INVALID KEY : {}", key);
			}
		}

		// ----------- vertical movement --------------------
		let time2 = View::sdl_get_tickets();
		if time2 - time1 > game::WAIT_TIME {
			if game.is_possible_movement(0, 1) {
				game.pos_y += 1;
			} else {
				game.board.store_piece(game.pos_x, game.pos_y);
				game.board.delete_possible_lines();

				if game.board.is_game_over() {
					if !game.restart() { break; }
				}
				game.create_new_piece();
			}

			time1 = View::sdl_get_tickets();
		}
	}

	View::clean_up();
}
