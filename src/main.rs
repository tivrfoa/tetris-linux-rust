mod view;
type View = view::View;

mod game;
mod board;
mod piece;
mod command;
mod SDL;
mod c_api;

use command::Command;
use SDL::SDL_Event;

fn main() {
	//println!("Hello, world!");
	
	// game::Game::get_rand(3, 5);

	//View::test_message_box();

	View::init_graph();
	let screen_height = View::get_screen_height();
	//println!("Screen height = {}", screen_height);
	let game = game::Game::new(screen_height);

	let sdl_event = SDL_Event {
		type_: 0
	};

	let mut time1 = View::sdl_get_tickets();
	//println!("time1 = {}", time1);

	let mut quit = false;

	while !quit {
		game.draw_scene();

		let key = Command::poll_key(&sdl_event);
		println!("Key is = {}", key);
		break;
	}
}
