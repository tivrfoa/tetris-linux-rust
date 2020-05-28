#[link(name = "hello", kind = "static")]
extern "C" {
    pub fn messageBox() -> i32;
    pub fn initGraph();

    pub fn drawBlock(pX1: i32, pY1: i32, pX2: i32, pY2: i32, pC: color);
    pub fn clearScreen  ();
    pub fn getScreenHeight () -> i32;
    pub fn updateScreen   ();
    pub fn cleanUp        ();
    pub fn loadBackGround ();

    pub fn pollkey(event: &mut SDL_Event) -> i32;
    pub fn SDLGetTickets() -> u64;
    pub fn SDL_PollEvent(event: &mut SDL_Event) -> i32;
}

use crate::SDL::SDL_Event;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum color {ZERO, BLACK, RED, GREEN, BLUE, CYAN, MAGENTA,
    YELLOW, WHITE, COLOR_MAX}


use crate::piece::PieceType;
pub fn get_color(piece_type: PieceType) -> color {
    match piece_type {
        //PieceType::ZERO => panic!("Invalid type for color: "),
        PieceType::ZERO => color::WHITE,
        PieceType::I => color::WHITE,
        PieceType::L => color::RED,
        PieceType::LM => color::GREEN,
        PieceType::N => color::BLUE,
        PieceType::NM => color::CYAN,
        PieceType::S => color::MAGENTA,
        PieceType::T => color::YELLOW,
    }
}