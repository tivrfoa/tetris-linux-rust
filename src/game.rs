
const WAIT_TIME: usize = 700;

use crate::board::Board;
use crate::piece::{
    Piece,
    PieceType,
};
use crate::view::View;
use crate::command::Command;

use rand::{thread_rng, Rng};

pub struct Game {
    board: Board,
    next_piece: Piece,
    view: View,
    command: Command,
    pos_x: usize,
    pos_y: usize,
    next_pos_x: usize,
    next_pos_y: usize,
    piece_type: PieceType,
    next_piece_type: PieceType,
    rotation: i32,
    next_rotation: i32,
    m_screen_height: i32,
}

impl Game {

    pub fn new(view: View, command: Command, _m_screen_height: i32) -> Self {

        let m_screen_height: i32 = 480;

        let piece_type      = Piece::get_piece(Game::get_rand(1, 7));
        let next_piece_type = Piece::get_piece(Game::get_rand(1, 7));
        let rotation        = Game::get_rand(0, 3);
        let next_rotation   = Game::get_rand(0, 3);

        let piece           = Piece::new(piece_type, rotation);
        let next_piece      = Piece::new(next_piece_type, next_rotation);
        let board           = Board::new(piece, m_screen_height as usize);

        let pos_x           = Board::get_pos_x() as usize;
        let pos_y           = Piece::get_y_initial_position(&board.piece) as usize;

        let next_pos_x = 10;
        let next_pos_y = 0;

        Game {
            board,
            next_piece,
            view,
            command,
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
        let mut rng = thread_rng();
        let x: i32 = rng.gen();
        let x = x % (b - a + 1) + a;
        // println!("{}", x);

        x
    }

    pub fn create_new_piece(&mut self) {
        let mut new_piece = Piece::new(self.next_piece_type, self.next_rotation);
        std::mem::swap(&mut new_piece, &mut self.next_piece);
        self.board.update_current_piece(new_piece);

        self.pos_x = Board::get_pos_x() as usize;
        self.pos_y = self.board.piece.get_y_initial_position() as usize;
        self.next_piece_type = Piece::get_piece(Game::get_rand(1, 7));
        self.next_rotation = Game::get_rand(0, 3);
    }

    pub fn draw_piece(&self, x: usize, y: usize, piece_to_draw: &Piece) { 
        // Board::draw_piece(&self.board, x, y, piece_to_draw);
        self.board.draw_piece(x, y, piece_to_draw);
    }

    pub fn draw_board(&self) {
        // Board::draw_board(&self.board, self.m_screen_height);
        self.board.draw_board(self.m_screen_height);
    }

    pub fn draw_scene(&self) {
        View::clear_screen();
        View::load_background();
        self.draw_board();
        self.draw_piece(self.pos_x, self.pos_y, &self.board.piece);
        self.draw_piece(self.pos_x, self.pos_y, &self.get_next_piece());

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