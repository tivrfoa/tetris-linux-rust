
pub const WAIT_TIME: u64 = 700;

use crate::board::Board;
use crate::piece::{
    Piece,
    PieceType,
};
use crate::view::View;

use rand::Rng;

pub struct Game {
    pub board: Board,
    next_piece: Piece,
    pub pos_x: i32,
    pub pos_y: i32,
    next_pos_x: i32,
    next_pos_y: i32,
    piece_type: PieceType,
    next_piece_type: PieceType,
    rotation: i32,
    next_rotation: i32,
    m_screen_height: i32,
}

impl Game {

    pub fn new(_m_screen_height: i32) -> Self {

        let m_screen_height: i32 = 480;

        let piece_type      = Piece::get_piece(Game::get_rand(1, 7));
        let next_piece_type = Piece::get_piece(Game::get_rand(1, 7));
        let rotation        = Game::get_rand(0, 3);
        let next_rotation   = Game::get_rand(0, 3);

        let piece           = Piece::new(piece_type, rotation);
        let next_piece      = Piece::new(next_piece_type, next_rotation);
        let board           = Board::new(piece, m_screen_height);

        let pos_x           = Board::get_pos_x();
        let pos_y           = Piece::get_y_initial_position(&board.piece);

        let next_pos_x = 10;
        let next_pos_y = 0;

        Game {
            board,
            next_piece,
            pos_x,
            pos_y,
            next_pos_x,
            next_pos_y,
            piece_type,
            next_piece_type,
            rotation,
            next_rotation,
            m_screen_height,
        }
    }

    pub fn get_rand(a: i32, b: i32) -> i32 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(a, b+1);
        assert!(x >= a && x <= b, "{} should be >= {} and <= {}", x, a, b);
        // println!("{}", x);

        x
    }

    pub fn is_possible_movement(&self, x: i32, y: i32) -> bool {
        self.board.is_possible_movement(self.pos_x + x, self.pos_y + y)
    }

    pub fn create_new_piece(&mut self) {
        let mut new_piece = Piece::new(self.next_piece_type, self.next_rotation);
        std::mem::swap(&mut new_piece, &mut self.next_piece);
        self.board.update_current_piece(new_piece);

        self.pos_x = Board::get_pos_x();
        self.pos_y = self.board.piece.get_y_initial_position();
        self.next_piece_type = Piece::get_piece(Game::get_rand(1, 7));
        self.next_rotation = Game::get_rand(0, 3);
    }

    pub fn draw_piece(&self, x: i32, y: i32, piece_to_draw: &Piece) { 
        // Board::draw_piece(&self.board, x, y, piece_to_draw);
        self.board.draw_piece(x, y, piece_to_draw); // STACK 3
    }

    pub fn draw_board(&self) {
        // Board::draw_board(&self.board, self.m_screen_height);
        self.board.draw_board(self.m_screen_height);
    }

    pub fn draw_scene(&self) {
        View::clear_screen();
        View::load_background();
        self.draw_board();
        self.draw_piece(self.pos_x, self.pos_y, &self.board.piece); // STACK 4
        self.draw_piece(self.next_pos_x, self.next_pos_y, &self.get_next_piece());

        View::update_screen();
    }

    pub fn get_next_piece(&self) -> Piece {
        Piece::new(self.next_piece_type, self.next_rotation)
    }

    pub fn restart(&mut self) -> bool {
        let response = View::message_box();
        if response > 0 {
            self.board.clear_board();
            true
        } else {
            false
        }
    }
}