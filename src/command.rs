use crate::SDL::SDL_Event;
use crate::c_api::*;

pub struct Command;

impl Command {

    pub fn poll_key(event: &mut SDL_Event) -> i32 {
        unsafe {
            pollkey(event)
        }
    }

    pub fn sdl_poll_event(event: &mut SDL_Event) -> i32 {
        unsafe {
            SDL_PollEvent(event)
        }
    }
}