use crate::SDL::SDL_Event;
use crate::c_api::*;

pub struct Command;

impl Command {

    fn poll_key(event: &SDL_Event) -> i32 {
        unsafe {
            pollKey(event)
        }
    }
    fn is_key_down(p_key: i32) -> i32 {
        unsafe {
            isKeyDown(p_key)
        }
    }
}