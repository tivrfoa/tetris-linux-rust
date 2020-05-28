const BOARD_LINE_WIDTH: i32 = 6;          // Width of each of the two lines that delimit the board
const BLOCK_SIZE: i32 = 20;               // Width and Height of each block of a piece
const BOARD_POSITION: i32 = 512;          // Center position of the board from the left of the screen
const BOARD_WIDTH: i32 = 10;              // Board width in blocks
const BOARD_HEIGHT: i32 = 20;             // Board height in blocks
const PIECE_BLOCKS: i32 = 5;              // Number of horizontal and vertical blocks of a matrix piece

pub struct Board {
    pub piece: Piece,
    m_screen_height: i32,
    m_board: [[PieceType;BOARD_HEIGHT as usize];BOARD_WIDTH as usize],
    // is_block_free: fn(pX: usize, pY: usize) -> bool,
}

const POS_FREE: PieceType = PieceType::ZERO;

use crate::piece::{
    Piece,
    PieceType
};

use crate::c_api::{
    color,
    get_color,
};

use crate::view::View;

impl Board {
    pub fn new (piece: Piece, _m_screen_height: i32) -> Self {
        Board {
            piece,
            m_screen_height: 480,
            m_board: [[PieceType::ZERO;BOARD_HEIGHT as usize];BOARD_WIDTH as usize],
        }
    }

    pub fn get_x_pos_in_pixels (pos: i32) -> i32 {
        (BOARD_POSITION - (BLOCK_SIZE * (BOARD_WIDTH / 2)) ) + (pos * BLOCK_SIZE)
    }

    pub fn get_y_pos_in_pixels (&self, pos: i32) -> i32 {
        // println!("get_y_pos_in_pixels -> pPos = {}", pPos);
        //panic!("test");
        self.m_screen_height - (BLOCK_SIZE * BOARD_HEIGHT) + (pos * BLOCK_SIZE)
    }

    pub fn is_block_free (&self, x: usize, y: usize) -> bool {
        self.m_board[x][y] == PieceType::ZERO
    }

    ///
    /// Checks collision with pieces already stored in the board or the board limits
    /// This is just to check the 5x5 blocks of a piece with the appropriate area in the board
    pub fn is_possible_movement (&self, x: i32, y: i32) -> bool {
        let mut board_x;
        let mut board_y;

        for i in 0..PIECE_BLOCKS {
            board_x = x + i;
            for j in 0..PIECE_BLOCKS {
                board_y = y + j;
                // Check if the piece is outside the limits of the board
                if board_x < 0 || board_x > BOARD_WIDTH - 1 || board_y > BOARD_HEIGHT - 1 {
                    if self.piece.m_piece[i as usize][j as usize] != 0 {
                        return false;
                    }
                }
                // Check if the piece have collisioned with a block already stored in the map
                if board_y >= 0 {
                    if self.piece.m_piece[i as usize][j as usize] != 0 &&
                            !self.is_block_free(board_x as usize, board_y as usize) {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Store each block of the piece into the board
    pub fn store_piece (&mut self, x: i32, y: i32) {
        let mut board_x;
        let mut board_y;
        
        for i in 0..PIECE_BLOCKS {
            board_x = x + i;
            for j in 0..PIECE_BLOCKS {
                board_y = y + j;
                // Store only the blocks of the piece that are not holes
                if self.piece.m_piece[i as usize][j as usize] != 0 {
                    self.m_board[board_x as usize][board_y as usize] = self.piece.piece_type;
                }
            }
        }
    }

    pub fn delete_possible_lines (&mut self) {
        for i in 0..BOARD_HEIGHT {
            let mut col = 0;
            while col < BOARD_WIDTH {
                if self.m_board[col as usize][i as usize] == POS_FREE { break; }
                col += 1;
            }
            if col == BOARD_WIDTH { self.delete_line(i as usize); }
        }
    }

    pub fn is_game_over (&self) -> bool {
        //If the first line has blocks, then, game over
        for i in 0..BOARD_WIDTH {
            if self.m_board[i as usize][0] != POS_FREE { return true; }
        }
    
        false
    }

    /// Moves all the upper lines one row down
    pub fn delete_line (&mut self, y: usize) {
        for j in (1..=y).rev() {
            for i in 0..BOARD_WIDTH {
                self.m_board[i as usize][j] = self.m_board[i as usize][j-1];
            }
        }
    }

    pub fn clear_board(&mut self) {
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                self.m_board[i as usize][j as usize] = PieceType::ZERO;
            }
        }
    }

    pub fn stored_piece_type(&self, x: usize, y: usize) -> PieceType {
        self.m_board[x][y]
    }

    pub fn get_pos_x() -> i32 {
        let tmp = (BOARD_WIDTH / 2) + Piece::get_x_initial_position();
        // println!("tmp = {}", tmp); // Initial value 3

        tmp
    }

    pub fn update_current_piece(&mut self, piece: Piece) {
        self.piece = piece;
    }

    pub fn draw_piece(&self, x: i32, y: i32, piece_to_draw: &Piece) {        
        let m_color = get_color(piece_to_draw.piece_type);
        let x_pos_in_pixels = Board::get_x_pos_in_pixels(x);
        let y_pos_in_pixels = self.get_y_pos_in_pixels(y); // STACK 2

        for i in 0..PIECE_BLOCKS {
            for j in 0..PIECE_BLOCKS {
                if piece_to_draw.m_piece[i as usize][j as usize] != 0 {
                    View::draw_block((x_pos_in_pixels + i * BLOCK_SIZE) as i32,
                                     (y_pos_in_pixels + j * BLOCK_SIZE) as i32,
                                     BLOCK_SIZE as i32 - 1,
                                     BLOCK_SIZE as i32 - 1,
                                     m_color);
                }
            }
        }
    }

    pub fn draw_board(&self, m_screen_height: i32) {
        let mut x1 = BOARD_POSITION - (BLOCK_SIZE * (BOARD_WIDTH / 2)) - 1;
        let x2 = BOARD_POSITION + (BLOCK_SIZE * (BOARD_WIDTH / 2));
        let y  = m_screen_height - BLOCK_SIZE * BOARD_HEIGHT;

        View::draw_block(x1 - BOARD_LINE_WIDTH, y, BOARD_LINE_WIDTH, m_screen_height - 1, color::BLUE);
        View::draw_block(x2, y, BOARD_LINE_WIDTH, m_screen_height - 1, color::BLUE);
        View::draw_block(x1, m_screen_height, BLOCK_SIZE * BOARD_WIDTH, BOARD_LINE_WIDTH, color::BLUE);
        
        x1 += 1;

        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                let m_color = get_color(Board::stored_piece_type(self, i as usize, j as usize));
                if !self.is_block_free(i as usize, j as usize) {
                    View::draw_block(x1 + i * BLOCK_SIZE,
                                     y + j * BLOCK_SIZE,
                                     BLOCK_SIZE - 1,
                                     BLOCK_SIZE - 1,
                                     m_color);
                }
            }
        }
    }
}