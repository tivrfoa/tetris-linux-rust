

const SIZE: usize = 5;

const I: [[i32; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0]
];

const L: [[i32; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 1, 0],
    [0, 0, 0, 0, 0]
];

const LM: [[i32; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 1, 0, 0],
    [0, 0, 0, 0, 0]
];

const N: [[i32; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0]
];

const NM: [[i32; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0 ,1, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0]
];

const S: [[i32; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 1, 0],
    [0, 0, 1, 1, 0],
    [0, 0, 0, 0, 0]
];

const T: [[i32; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0]
];

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PieceType {
    ZERO = 0,
    I = 1,
    L = 2,
    LM = 3,
    N = 4,
    NM = 5,
    S = 6,
    T = 7,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub m_piece: [[i32; 5]; 5],
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_rotation: i32) -> Self {
        let mut m_piece = [[0i32; 5]; 5];
        let mut piece = Piece {
            piece_type,
            m_piece,
        };
        Piece::set_piece(&mut piece, piece_type);
        Piece::rotate_piece(&mut piece, piece_rotation);

        piece
    }

    pub fn get_x_initial_position() -> i32 {
        -2
    }

    pub fn get_y_initial_position(&self) -> i32 {
        for i in 0..SIZE {
            if self.m_piece[3][i] == 1 { return -3; }
            if self.m_piece[4][i] == 1 { return -4; }
        }
        -2
    }

    pub fn rotate_piece(&mut self, piece_rotation: i32) {
        let mut count = 0;
        while count < piece_rotation {
            for i in 0..SIZE - 1 {
                for j in i..SIZE - i - 1 {
                    let temp = self.m_piece[i][j];
                    self.m_piece[i][j]                       = self.m_piece[SIZE - j - 1][i];
                    self.m_piece[SIZE - j - 1][i]            = self.m_piece[SIZE - i - 1][SIZE - j - 1];
                    self.m_piece[SIZE - i - 1][SIZE - j - 1] = self.m_piece[j][SIZE - i - 1];
                    self.m_piece[j][SIZE - i - 1]            = temp;
                }
            }
            count += 1;
        }
    }

    pub fn set_piece(&mut self, piece_type: PieceType) {
        match piece_type {
            PieceType::ZERO => (),
            PieceType::I => Piece::copy(&I, &mut self.m_piece),
            PieceType::L => Piece::copy(&L, &mut self.m_piece),
            PieceType::LM => Piece::copy(&LM, &mut self.m_piece),
            PieceType::N => Piece::copy(&N, &mut self.m_piece),
            PieceType::NM => Piece::copy(&NM, &mut self.m_piece),
            PieceType::S => Piece::copy(&S, &mut self.m_piece),
            PieceType::T => Piece::copy(&T, &mut self.m_piece),
        }
    }

    fn copy(src: &[[i32; 5]; 5], dest: &mut [[i32; 5]; 5]) {
        for i in 0..5 {
            for j in 0..5 {
                dest[i][j] = src[i][j];
            }
        }
    }

    pub fn get_piece(piece_type_value: i32) -> PieceType {
        match piece_type_value {
            1 => PieceType::I,
            2 => PieceType::L,
            3 => PieceType::LM,
            4 => PieceType::N,
            5 => PieceType::NM,
            6 => PieceType::S,
            7 => PieceType::T,
            _ => panic!("Invalid piece type value."),
        }
    }
}