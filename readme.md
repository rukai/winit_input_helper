# Winit Input Helper

[![Crates.io](https://img.shields.io/crates/v/winit_input_helper.svg)](https://crates.io/crates/winit_input_helper)
[![Docs](https://docs.rs/winit_input_helper/badge.svg)](https://docs.rs/winit_input_helper)

Processes and stores winit events, allowing input state to be queried at any time.

## How to use

Each event is passed to the `WinitInputHelper` via the `update` method.

The current input state can then be accessed via methods such as `key_pressed`, `key_released`, `key_held`, `mouse`, `mouse_diff` etc.

To see all available methods look at [docs.rs](https://docs.rs/winit_input_helper)

```rust
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
        // It will return true if you should render.
        if self.input.process_window_event(&event) {
            // render();

            // If you want to render every frame, remember to call window.request_redraw() in ApplicationHandler.about_to_wait().
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        // pass in events
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
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut App {
            input: WinitInputHelper::new(),
            window: None,
        })
        .unwrap();
}

```

## Examples

* To run example natively, run `cargo run --example example`
* To run example in wasm, run `cargo run-wasm --example example`

## Publishing a new version

In order to avoid forcing the user to enable the default winit backends, winit_input_helper sets its winit dependency to `default-features = false`.
This complicates the publishing procedure a little because winit cannot compile without any backends enabled.

So to publish we run: `cargo publish --features winit/default`
