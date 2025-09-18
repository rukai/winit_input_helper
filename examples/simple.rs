//! The simplest example, supporting only desktop applications.

use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};
use winit_input_helper::WinitInputHelper;

struct App {
    window: Option<Window>,
    input: WinitInputHelper,
}

impl ApplicationHandler for App {
    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // Pass every event to the WinitInputHelper.
        // It will return true if it receives a RequestedRedraw event: you should then render.
        if self.input.process_window_event(&event) {
            // render();

            // If you want to render every frame, remember to call window.request_redraw() in ApplicationHandler.about_to_wait().
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        self.input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.input.end_step();

        // We do not call window.request_redraw() here because we have nothing to render anyways

        if self.input.key_released(KeyCode::KeyQ)
            || self.input.close_requested()
            || self.input.destroyed()
        {
            println!("The application was requsted to close or the 'Q' key was pressed, quiting the application");
            event_loop.exit();
            return;
        }

        if self.input.key_pressed(KeyCode::KeyW) {
            println!("The 'W' key (US layout) was pressed on the keyboard");
        }
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        self.input.step();
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.window = Some(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );
        }
    }
}

fn main() {
    // Create an event loop, initialize the app, and run it
    // Immediately, .resume() will be called
    // Then, every window event will trigger a .window_event() call
    // We set the control flow to "Poll" so that a .window_event() call is triggered
    // periodically even if there is no input, so that our app can continue to update
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut App {
            input: WinitInputHelper::new(),
            window: None,
        })
        .unwrap();
}
