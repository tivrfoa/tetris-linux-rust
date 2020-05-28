use crate::c_api::*;

pub struct View;

impl View {

    pub fn draw_block(x1: i32, y1: i32, x2: i32, y2: i32, c: color) {
        unsafe {
            drawBlock(x1, y1, x2, y2, c);
        }
    }

    pub fn clear_screen() {
        unsafe {
            clearScreen();
        }
    }

    pub fn get_screen_height() -> i32 {
        unsafe {
            getScreenHeight()
        }
    }

    pub fn init_graph() {
        unsafe {
            initGraph();
        }
    }

    pub fn update_screen() {
        unsafe {
            updateScreen();
        }
    }

    pub fn clean_up() {

    }

    pub fn load_background() {
        unsafe {
            loadBackGround();
        }
    }

    pub fn message_box() -> i32 {
        unsafe {
            messageBox()
        }
    }

    pub fn load_test() {

    }

    pub fn sdl_get_tickets() -> u64 {
        unsafe {
            let tmp = SDLGetTickets();
            tmp
        }
    }


    pub fn test_message_box() {
        unsafe {
            println!("{}", messageBox());
        }
    }

}