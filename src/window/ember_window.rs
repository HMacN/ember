use std::thread::JoinHandle;
use crate::window::winit_window_wrapper::{create_window, WinItWindowWrapper};

pub struct EmberWindow {
    window: Option<WinItWindowWrapper>,
    pub handle: Option<JoinHandle<()>>,
}

impl EmberWindow {

    pub fn new() -> EmberWindow {
        EmberWindow {
            window: None,
            handle: None
        }
    }

    pub fn run(&mut self) -> () {
        let _ = create_window();
    }

    fn close(&mut self) {

    }
}