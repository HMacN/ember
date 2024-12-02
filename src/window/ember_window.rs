use std::thread;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::window::winit_window_wrapper::WinItWindowWrapper;

pub struct EmberWindow {
    window: Option<WinItWindowWrapper>
}

impl EmberWindow {

    pub fn new() -> EmberWindow {
        return EmberWindow {
            window: None,
        };
    }

    // TODO:
    // Move this (and all other code that causes EmberWindow to depend on WinIt) into a new class
    // that isn't exposed to the user.
    pub fn run(&mut self) -> () {
        thread::spawn(|| {
            let event_loop = EventLoop::new().unwrap();
            event_loop.set_control_flow(ControlFlow::Poll);
            let mut state = WinItWindowWrapper::new();
            let _ = event_loop.run_app(&mut state);
        });
    }

    fn close(&mut self) {

    }
}