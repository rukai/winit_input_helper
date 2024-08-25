//! The simplest example, supporting only desktop applications.
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};
use winit_input_helper::WinitInputHelper;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop
        .run_app(&mut App {
            input: WinitInputHelper::new(),
            _window: None,
        })
        .unwrap();
}

struct App {
    input: WinitInputHelper,
    _window: Option<Window>,
}

impl ApplicationHandler for App {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        self.input.step();
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self._window.is_none() {
            let window = event_loop.create_window(Default::default()).unwrap();
            self._window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.input.process_window_event(&event);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        self.input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.input.end_step();

        if self.input.key_released(KeyCode::KeyQ)
            || self.input.close_requested()
            || self.input.destroyed()
        {
            println!("The application was requsted to close or the 'Q' key was pressed, quiting the application");
            event_loop.exit();
            // If you don't drop window, the app won't quit before any other window event.
            // https://github.com/rust-windowing/winit/issues/3673
            self._window = None;
            return;
        }

        if self.input.key_pressed(KeyCode::KeyW) {
            println!("The 'W' key (US layout) was pressed on the keyboard");
        }

        // You are expected to control your own timing within this block.
        // Usually via rendering with vsync.
        // render();
    }
}
