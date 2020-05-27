#[repr(C)]
pub union SDL_Event {
    pub type_: i32,
    padding: [i8; 56],
}

pub const SDL_QUIT: i32 = 256;
pub const SDLK_LEFT: i32 = 1073741904;
pub const SDLK_RIGHT: i32 = 1073741903;
pub const SDLK_DOWN: i32 = 1073741905;
pub const SDLK_SPACE: i32 = 32;
pub const SDLK_UP: i32 = 1073741906;