mod view;
type View = view::View;

mod game;
mod board;
mod piece;
mod command;
mod SDL;
mod c_api;

use command::Command;
use SDL::*;

fn main() {
	//println!("Hello, world!");
	
	// game::Game::get_rand(3, 5);

	//View::test_message_box();

	View::init_graph();
	let screen_height = View::get_screen_height();
	//println!("Screen height = {}", screen_height);
	let mut game = game::Game::new(screen_height);

	let sdl_event = SDL_Event {
		type_: 0
	};

	let mut time1 = View::sdl_get_tickets();
	//println!("time1 = {}", time1);

	let mut quit = false;

	'main_loop:
	while !quit {
		game.draw_scene();

		let key = Command::poll_key(&sdl_event);
		if key != -1 {
			println!("Key is = {}", key); // 32
		}

		let mut event = Command::sdl_poll_event(&sdl_event);
		while event != 0 {
			unsafe {
				println!("sdl_event.type_ = {}", sdl_event.type_);
				if sdl_event.type_ == SDL_QUIT {
					quit = true;
				}
			}
			event = Command::sdl_poll_event(&sdl_event);
		}
		// println!("qnt = {}", qnt);

		/*
event->type = 1024
Key is = 1073741906 -> up
Key is = 1073741905 -> down
Key is = 1073741904 -> left
Key is = 1073741903 -> right
Key is = 32
		*/
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
				game.board.deletePossibleLines();

				if game.board.isGameOver() {
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
				game.board.deletePossibleLines();

				if game.board.isGameOver() {
					if !game.restart() { break; }
				}
				game.create_new_piece();
			}

			time1 = View::sdl_get_tickets();
		}
	}

	View::clean_up();
}
