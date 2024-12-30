use std::thread;
use crate::window::winit_window_wrapper::{foo, WinItWindowWrapper};

pub struct EmberWindow {
    window: Option<WinItWindowWrapper>,
}

impl EmberWindow {

    pub fn new() -> EmberWindow {
        EmberWindow {
            window: Some(WinItWindowWrapper::new())
        }
    }

    pub fn run(&mut self) -> () {
        thread::spawn(foo);
    }

    fn close(&mut self) {

    }
}