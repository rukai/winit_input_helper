//! The simplest example, supporting only desktop applications.

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};
use winit_input_helper::{WinitInputApp, WinitInputHelper, WinitInputUpdate};

struct State {
    window: Option<Window>,
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let state = State { window: None };
    let mut winit_input = WinitInputApp::new(state);

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut winit_input).unwrap();
}

impl ApplicationHandler<()> for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }
}

impl WinitInputUpdate for State {
    fn update(&mut self, event_loop: &ActiveEventLoop, input: &WinitInputHelper) {
        if input.key_released(KeyCode::KeyQ) || input.close_requested() || input.destroyed() {
            println!("The application was requsted to close or the 'Q' key was pressed, quiting the application");
            event_loop.exit();
            return;
        }

        if input.key_pressed(KeyCode::KeyW) {
            println!("The 'W' key (US layout) was pressed on the keyboard");
        }

        // You are expected to control your own timing within this block.
        // Usually via rendering with vsync.
        // render();

        // We aren't rendering anything, but we still want inputs to be processed.
        self.window.as_ref().unwrap().request_redraw();
    }
}
