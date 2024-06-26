use winit::window::{Window, WindowId};
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::event::{StartCause, WindowEvent};
use std::time::Instant;

pub struct WinItWindowWrapper {
    window: Option<Window>,
}

impl WinItWindowWrapper {
    pub fn new() -> WinItWindowWrapper {
        let new_window = WinItWindowWrapper {
            window: None,
        };

        return new_window;
    }
}

impl ApplicationHandler for WinItWindowWrapper {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if self.window.is_none() {
            self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap())
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.new_events(event_loop, StartCause::ResumeTimeReached {start: Instant::now(), requested_resume: Instant::now() });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        if event == (WindowEvent::CloseRequested) {
            self.window = None;
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}
