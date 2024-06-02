use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

pub struct EmberWindow {

}

impl EmberWindow {

    pub fn new() -> EmberWindow {
        return EmberWindow {};
    }

    pub fn run(self) -> () {
        let event_loop = EventLoop::new().unwrap();
        let mut state = WindowState::default();
        let _ = event_loop.run_app(&mut state);
    }
}

#[derive(Default)]
struct WindowState {
    window: Option<Window>
}

impl WindowState {

}

impl ApplicationHandler for WindowState {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap())
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.new_events(event_loop, StartCause::ResumeTimeReached {start: Instant::now(), requested_resume: Instant::now() });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {

    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}