//! The simplest example, supporting only desktop applications.

use winit::application::ApplicationHandler;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

struct App {
    window: Option<Window>,
    winit_input_helper: WinitInputHelper,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let None = self.window {
            self.window = Some(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );
        }
    }
    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // Pass every event to the WinitInputHelper.
        // It will return true if it receives a RequestedRedraw event: you should then render.
        if self.winit_input_helper.process_window_event(&event) {
            // render();

            // If you want to render every frame, remember to call window.request_redraw() in ApplicationHandler.about_to_wait().
        }
    }
    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.winit_input_helper.process_device_event(&event);
    }
    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.winit_input_helper.process_about_to_wait();

        // We do not call window.request_redraw() here because we have nothing to render anyways
    }
    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        self.winit_input_helper.process_new_events();
    }
}
fn main() {
    // WinitInputHelper is initialized once at the start of the app
    let winit_input_helper = WinitInputHelper::new();

    // Create an event loop, initialize the app, and run it
    // Immediately, .resume() will be called
    // Then, every window event will trigger a .window_event() call
    // We set the control flow to "Poll" so that a .window_event() call is triggered
    // periodically even if there is no input, so that our app can continue to update
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut App {
            winit_input_helper,
            window: None,
        })
        .unwrap();
}
