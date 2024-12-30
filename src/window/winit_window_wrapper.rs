use winit::window::{Window, WindowId};
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::event::{StartCause, WindowEvent};
use std::time::Instant;


// TODO: Switch over to using winit_modular (https://docs.rs/winit-modular/latest/winit_modular/index.html) to avoid hogging the main thread.
pub struct WinItWindowWrapper {
    window: Option<Window>,
}

impl WinItWindowWrapper {
    fn new() -> WinItWindowWrapper {
        let new_window = WinItWindowWrapper {
            window: None,
        };

        new_window
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

pub fn create_window() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut state = WinItWindowWrapper::new();
    let _ = event_loop.run_app(&mut state);
}