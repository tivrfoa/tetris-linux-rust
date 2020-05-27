mod view;
type View = view::View;

mod game;
mod board;
mod piece;
mod command;
mod SDL;
mod c_api;

fn main() {
	//println!("Hello, world!");
	
	// game::Game::get_rand(3, 5);

	// unsafe { print(); }

	// View::draw_block();
	View::init_graph();

	//View::test_message_box();

	
	let screen_height = View::get_screen_height();
	println!("Screen height = {}", screen_height);
	/*let game = game::Game::new(screen_height);*/

	//let mut time1 = View::sdl_get_tickets();
	//println!("time1 = {}", time1);

	//let mut quit = false;

	/*while !quit {

	}*/
}
