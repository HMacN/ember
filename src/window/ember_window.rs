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

    pub fn run(&mut self) -> () {
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
        // TODO: Create a new window in here, as per https://docs.rs/winit/latest/winit/changelog/index.html

        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap())
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // TODO: Perhaps call new_events() here if the window hasn't been created yet?  Need to provide a StartCause Enum as per https://docs.rs/winit/latest/winit/event/enum.StartCause.html
        // self.new_events(event_loop, StartCause::ResumeTimeReached {start: de});
        todo!()
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        todo!()
    }
}