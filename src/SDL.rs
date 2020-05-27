#[repr(C)]
pub union SDL_Event {
    pub type_: i32,
    padding: [i8; 56],
}