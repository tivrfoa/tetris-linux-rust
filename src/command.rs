use crate::SDL::SDL_Event;
use crate::c_api::*;

pub struct Command;

impl Command {

    pub fn poll_key(event: &SDL_Event) -> i32 {
        unsafe {
            pollkey(event)
        }
    }
    pub fn is_key_down(p_key: i32) -> i32 {
        unsafe {
            isKeyDown(p_key)
        }
    }

    pub fn sdl_poll_event(event: &SDL_Event) -> i32 {
        unsafe {
            SDL_PollEvent(event)
        }
    }
}