mod view;
type View = view::View;

mod game;
mod board;
mod piece;
mod command;
mod SDL;
mod c_api;

fn main() {
	println!("Hello, world!");
	
	game::Game::get_rand(3, 5);

	// unsafe { print(); }

	// View::draw_block();
	View::init_graph();
}
